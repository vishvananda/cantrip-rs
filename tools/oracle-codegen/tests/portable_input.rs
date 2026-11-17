use cantrip_oracle_codegen::{corpus, cantrip, input::Record};
use std::{path::{Path,PathBuf},sync::atomic::{AtomicU64,Ordering}};
struct Files(PathBuf);
impl Files {fn new()->Self {static NEXT:AtomicU64=AtomicU64::new(0);let path=std::env::temp_dir().join(format!("codegen-input-{}-{}",std::process::id(),NEXT.fetch_add(1,Ordering::Relaxed)));std::fs::create_dir(&path).unwrap();Self(path)}fn path(&self,name:&str)->PathBuf{self.0.join(name)}}
impl Drop for Files {fn drop(&mut self){let _=std::fs::remove_dir_all(&self.0);}}
fn fixture()->PathBuf{Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/records.jsonl")}
#[test]
fn standalone_ir_covers_single_and_multi_face_records_without_a_producer() {
    let cards=corpus::read_cards(&fixture()).unwrap();
    assert_eq!(cards.len(),7);
    let mut order=Vec::new();corpus::map_cards(&fixture(),|line,record|Ok((line,record.identity.names)),|line,row|{assert_eq!(line,row.0);order.push(row.1);Ok(())}).unwrap();
    assert_eq!(order,cards.iter().map(|r|r.identity.names.clone()).collect::<Vec<_>>());
    let bolt=cards.iter().find(|r|r.identity.names==["Lightning Bolt"]).unwrap();
    let code=cantrip::whole_module(&bolt.context,&bolt.card).unwrap();assert!(code.contains("g.damage"));
    for mut record in cards.into_iter().filter(|r|r.card.faces.len()>1) {
        assert!(record.context.primary_face.is_some());
        record.validate().unwrap();
        record.context.primary_face=None;assert!(record.validate().is_err());
        record.context.primary_face=Some("missing face".into());assert!(record.validate().is_err());
    }
}
#[test]
fn rejected_record_never_replaces_previous_generation() {
    let files=Files::new();let input=files.path("input.jsonl");let oracle=files.path("oracle.json");let output=files.path("rows.jsonl");
    let good=std::fs::read_to_string(fixture()).unwrap().lines().next().unwrap().to_owned();
    let mut bad:serde_json::Value=serde_json::from_str(&good).unwrap();
    bad["card"]["unknown_rule"]=serde_json::json!(true);
    std::fs::write(&input,format!("{good}\n{bad}\n")).unwrap();std::fs::write(&oracle,"[]").unwrap();std::fs::write(&output,"previous output").unwrap();
    let error=cantrip::planned::lane_rows(&input,&oracle,&output,None).unwrap_err().to_string();
    assert!(error.contains(":2:"),"{error}");assert!(error.contains("unknown or missing IR fields"),"{error}");
    assert_eq!(std::fs::read_to_string(output).unwrap(),"previous output");
}
#[test]
fn cli_rejects_versions_and_trailing_or_missing_records() {
    let files=Files::new();let input=files.path("input.jsonl");let good=std::fs::read_to_string(fixture()).unwrap().lines().next().unwrap().to_owned();
    for value in [format!("{good} extra"),format!("{good}\n\n"),good.replace("\"format_version\":1","\"format_version\":2")] {
        std::fs::write(&input,value).unwrap();
        assert!(!std::process::Command::new(env!("CARGO_BIN_EXE_cantrip-oracle-codegen")).args(["check-input",input.to_str().unwrap()]).output().unwrap().status.success());
    }
    let _:Record=serde_json::from_str(&good).unwrap();
}

#[test]
fn printed_basic_land_mana_does_not_add_a_second_intrinsic_activation() {
    use cantrip_oracle_codegen::{ir::*, mtg_types::{CardType, SubType}};
    let mut face=corpus::read_cards(&fixture()).unwrap().remove(0).card.faces.remove(0);
    face.name="Forest".into();
    face.characteristics.card_types=vec![CardType::Land];
    face.characteristics.subtypes=vec![SubType::Forest];
    face.characteristics.mana_cost=None;
    face.characteristics.power_toughness=None;
    face.abilities.clear();
    let unprinted=cantrip::module(&face).unwrap();
    let printed=serde_json::json!({"kind":"Activated","args":{
        "origin":"Ordinary","cost":{"parts":["TapSymbol"]},
        "limits":{"once":false,"once_each_turn":false,"sorcery":false,"condition":null,"activators":null,"keyword":null,"modifiers":[]},
        "announcement":{"targets":[],"modes":null},
        "body":{"instructions":[{"kind":"Effect","args":{"result":0,"operation":{"kind":"AddMana","args":{
            "players":{"kind":"One","args":{"kind":"AbilityController"}},
            "mana":{"Amount":{"color":"Green","amount":{"kind":"Constant","args":1}}},"restriction":null
        }}}}]}
    }});
    // Use the published tagged form of ManaProduction, including its concrete color.
    let mut printed=printed;
    printed["args"]["body"]["instructions"][0]["args"]["operation"]["args"]["mana"]=
        serde_json::to_value(ManaProduction::Amount{color:ManaColor::Green,amount:Expr::Constant(1)}).unwrap();
    face.abilities.push(serde_json::from_value(printed).unwrap());
    assert_eq!(cantrip::module(&face).unwrap(),unprinted);
    if let Ability::Activated{body,..}=&mut face.abilities[0] {
        if let Instruction::Effect{operation:Operation::AddMana{restriction,..},..}=&mut body.instructions[0] {
            *restriction=Some(ManaUse::Purpose(ManaPurpose::CumulativeUpkeep));
        }
    }
    assert_ne!(cantrip::module(&face).unwrap(),unprinted,"restricted mana is a distinct printed ability");
}

#[test]
fn live_discard_replacement_uses_effect_controller_only_on_capable_engines() {
    use cantrip_oracle_codegen::ir::*;
    let mut face=corpus::read_cards(&fixture()).unwrap().remove(0).card.faces.remove(0);
    face.abilities=vec![Ability::Replacement{
        active:ReplacementActive::Battlefield,
        event:ReplacementEvent::Discard{players:PlayerSet::One(PlayerRef::AbilityController),causes:None,card:None,effect_only:true},
        body:Program{instructions:vec![Instruction::Effect{result:ResultId(0),operation:Operation::ModifyEvent{change:EventChange::SetDestination(Destination::OwnersLibraryTop)}}]},
    }];
    let generated=cantrip::planned::with_features(&["effect-cause-controller"],||cantrip::module(&face)).unwrap();
    assert!(generated.contains("q.effect_controller().is_some()"));
    assert!(!generated.contains("proposal_cause"));
    assert!(cantrip::planned::with_features(&[],||cantrip::module(&face)).is_err());
}

fn native_face(name: &str) -> cantrip_oracle_codegen::ir::Card {
    corpus::read_cards(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/native_interface_records.jsonl"))
        .unwrap().into_iter().flat_map(|record|record.card.faces).find(|face|face.name==name).unwrap()
}

#[test]
fn linked_tokens_use_the_creating_ability_and_acquisition_without_broad_cause_matching() {
    let face=native_face("Tetravus");
    let generated=cantrip::module(&face).unwrap();
    assert!(generated.contains("tokens_created_by("));
    assert!(generated.contains("index: 0"));
    assert!(!generated.contains("Event::Entered"));
    assert!(cantrip::planned::without_features(&["linked-token-creations"],||cantrip::module(&face)).is_err());
    let mut ambiguous=face;
    ambiguous.abilities.push(ambiguous.abilities[2].clone());
    assert!(cantrip::module(&ambiguous).is_err(),"two creating abilities require an explicit unambiguous link");
}

#[test]
fn historical_deaths_project_only_the_first_surviving_destination() {
    let face=native_face("Krovikan Vampire");
    let generated=cantrip::module(&face).unwrap();
    assert!(generated.contains("q.found_all_after_move(old)"));
    assert!(generated.contains("became.is_none()"));
    assert!(cantrip::planned::without_features(&["history-destination-objects"],||cantrip::module(&face)).is_err());
}

#[test]
fn counter_removal_reads_the_recorded_remover_and_resulting_total() {
    let generated=cantrip::module(&native_face("Divine Intervention")).unwrap();
    assert!(generated.contains("Event::CountersRemoved { object, kind, amount, total, by, .. }"));
    assert!(generated.contains("by.as_ref().is_some_and(|by|"));
    assert!(generated.contains("*total == 0"));
    assert!(generated.contains("g.draw_game()"));
}

#[test]
fn a_chosen_actor_does_not_rebind_the_ability_controller_or_repeat_for_all_players() {
    let generated=cantrip::module(&native_face("Stardew Valley")).unwrap();
    assert!(generated.contains("choose::player(g, asker.controller, asker.source, among_"));
    assert!(generated.contains("Some(doer_"));
    assert!(!generated.contains("Asker::new(asker.source, doer_"));
    assert!(!generated.contains("bound_9000"));
}

#[test]
fn entry_sacrifice_counts_read_the_linked_historical_rows() {
    let face=native_face("Wood Elemental");
    let generated=cantrip::module(&face).unwrap();
    assert!(generated.contains("sacrificed_as_entered(q.state(), q.linked_to(asker.source))"));
    assert!(generated.contains("row.subtypes.has(q.words(asker.source).subtype(Subtype::Forest))"));
    assert!(!generated.contains("q.chars(row.id)"));
    assert!(cantrip::planned::without_features(&["entry-sacrifice-history"],||cantrip::module(&face)).is_err());
}

#[test]
fn supertype_changes_use_independent_native_flags_and_keep_capability_gating() {
    let face=native_face("Arcum's Weathervane");
    let generated=cantrip::module(&face).unwrap();
    assert!(generated.contains("EffectKind::AddSupertypes(Supertypes { basic: false, legendary: false, snow: true, world: false }"));
    assert!(generated.contains("EffectKind::LoseSupertypes(Supertypes { basic: false, legendary: false, snow: true, world: false }"));
    assert!(cantrip::planned::without_features(&["supertype-changes"],||cantrip::module(&face)).is_err());
}

#[test]
fn damage_floor_replaces_life_results_through_the_native_damage_authority() {
    let face=native_face("Ali from Cairo");
    let generated=cantrip::module(&face).unwrap();
    assert!(generated.contains("replacement(ReplacementKind::LifeChange"));
    assert!(generated.contains("damage_would_reduce_your_life_below::<{1}>"));
    assert!(generated.contains("reduces_your_life_to::<{1}>"));
    assert!(!generated.contains("ReplacementKind::Damage"));
    assert!(cantrip::planned::without_features(&["damage-life-results-floor"],||cantrip::module(&face)).is_err());
}

#[test]
fn chosen_landwalks_keep_the_snow_qualification_and_include_nonbasic_subtypes() {
    for (name, feature, method) in [("Barbarian Guides","snow-landwalk-all-types","Keyword::snow_landwalk_of("),
        ("Illusionary Presence","landwalk-all-types","Keyword::landwalk_of(")] {
        let face=native_face(name);
        let generated=cantrip::module(&face).unwrap();
        assert!(generated.contains(method),"{name}: {generated}");
        assert!(!generated.contains("match chosen_"));
        assert!(cantrip::planned::without_features(&[feature],||cantrip::module(&face)).is_err());
    }
}

fn prevention_body(face: &mut cantrip_oracle_codegen::ir::Card) -> &mut cantrip_oracle_codegen::ir::Program {
    use cantrip_oracle_codegen::ir::*;
    for ability in &mut face.abilities {
        let body = match ability { Ability::Activated { body,.. } | Ability::Spell { body,.. } => body, _ => continue };
        for step in &mut body.instructions {
            if let Instruction::Effect { operation: Operation::CreateReplacement { body,.. },.. } = step { return body; }
        }
    }
    panic!("expected a generated prevention")
}

#[test]
fn granted_portions_read_the_pending_damage_without_a_creation_time_snapshot() {
    for (name, words) in [("Dark Sphere","half, rounded down, of"),("Forcefield","all but 1 of")] {
        let face=native_face(name);
        let generated=cantrip::module(&face).unwrap();
        assert!(generated.contains("portion: Some(PreventionPortion"));
        assert!(generated.contains(words));
        let syntax=syn::parse_file(&generated).unwrap();
        let amount=syntax.items.iter().find_map(|item| match item {
            syn::Item::Fn(function) if function.sig.ident.to_string().ends_with("_amount") => Some(function), _=>None,
        }).unwrap();
        assert_eq!(amount.sig.inputs.len(),3,"the actual native amount interface");
        use quote::ToTokens;
        let body=amount.block.to_token_stream().to_string();
        assert!(body.contains("e . amount"));
        assert!(!body.contains("item"),"the future event supplies the amount");
        assert!(cantrip::planned::without_features(&["granted-prevention-portions"],||cantrip::module(&face)).is_err());
    }
}

#[test]
fn prevention_life_gain_is_part_of_the_same_damage_event() {
    for name in ["Reverse Damage","Hallow"] {
        let face=native_face(name);
        let generated=cantrip::module(&face).unwrap();
        assert!(generated.contains("gain_prevented: true"));
        assert!(!generated.contains("g.gain_life("));
        assert!(!generated.contains("then: Some("));
        assert!(cantrip::planned::without_features(&["prevention-life-results"],||cantrip::module(&face)).is_err());
    }
    let palm=cantrip::module(&native_face("Deflecting Palm")).unwrap();
    assert!(palm.contains("then: Some("));
    assert!(!palm.contains("gain_prevented: true"));
}

#[test]
fn portions_do_not_imply_optional_prevention_or_a_fixed_event_allowance() {
    use cantrip_oracle_codegen::ir::*;
    let mut face=native_face("Dark Sphere");
    let body=prevention_body(&mut face);
    body.instructions=vec![Instruction::Optional {decider: PlayerRef::AbilityController,body:body.clone()}];
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="prevention a player may decline"));
    let mut face=native_face("Dark Sphere");
    let body=prevention_body(&mut face);
    let Instruction::Effect{operation:Operation::ModifyEvent{change},..}=&mut body.instructions[0] else {panic!()};
    *change=EventChange::PreventSomeDamage(Expr::Constant(2));
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="a fixed per-event allowance on a granted prevention"));
}

#[test]
fn the_controller_only_gain_flag_does_not_claim_other_players_gains() {
    use cantrip_oracle_codegen::ir::*;
    let mut face=native_face("Reverse Damage");
    let body=prevention_body(&mut face);
    let Instruction::Effect {operation:Operation::GainLife{players,..},..}=&mut body.instructions[1] else {panic!()};
    *players=PlayerSet::All;
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="conditional or modified life gain from prevention in the damage event"));
}

#[test]
fn blocker_piles_use_the_declaration_hook_and_preserve_additional_block_capacity() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Camouflage");
    face.name = "An unrelated name".into();
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("g.blockers_instead_this_turn(item, 0)"));
    assert!(generated.contains("over.defender"));
    assert!(generated.contains("over.creatures"));
    assert!(generated.contains("most"));
    assert!(generated.contains("g.choose_at_random(unassigned.len())"));
    assert!(generated.contains("cast_only_if: Some("));
    assert!(cantrip::planned::without_features(&["replacement-block-declarations"], || cantrip::module(&face)).is_err());
    let Ability::Spell { body, .. } = &mut face.abilities[1] else { panic!() };
    let mut again = body.instructions[0].clone();
    let Instruction::Effect { result, .. } = &mut again else { panic!() };
    *result = ResultId(1);
    body.instructions.push(again);
    let twice = cantrip::module(&face).unwrap();
    assert!(twice.contains("g.blockers_instead_this_turn(item, 1)"));
    let Ability::Spell { body, .. } = &mut face.abilities[1] else { panic!() };
    let Instruction::Effect { operation: Operation::GameEffect { until, .. }, .. } = &mut body.instructions[0] else { panic!() };
    *until = Duration::EndOfCombat;
    assert!(cantrip::module(&face).is_err(), "the native hook only lasts this turn");
}

#[test]
fn turn_start_orientation_uses_the_first_tap_transition_and_rejects_new_objects() {
    let face = native_face("Rasputin Dreamweaver");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("history.entered_this_turn(permanent)"));
    assert!(generated.contains("history.select(taps).next()"));
    assert!(generated.contains("Some(Event::Untapped"));
    assert!(generated.contains("ZoneKind::PhasedOut"));
    assert!(!generated.contains("marks.untapped_as_turn_began"));
    assert!(cantrip::planned::without_features(&["untap-events"], || cantrip::module(&face)).is_err());
}

#[test]
fn counter_ceilings_are_state_based_and_not_placement_limits() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Rasputin Dreamweaver");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("counter_ceilings: &[(CounterKind::Dream, 7)]"));
    assert!(!generated.contains("add_counters_with_limit"));
    assert!(cantrip::planned::without_features(&["counter-ceilings"], || cantrip::module(&face)).is_err());
    let Ability::Static { changes, .. } = face.abilities.last_mut().unwrap() else { panic!() };
    let Change::Rule(RuleChange::CounterLimit { max, .. }) = &mut changes[0] else { panic!() };
    *max = Expr::Constant(-1);
    assert!(cantrip::module(&face).is_err());
    let Ability::Static { changes, .. } = face.abilities.last_mut().unwrap() else { panic!() };
    let Change::Rule(RuleChange::CounterLimit { max, .. }) = &mut changes[0] else { panic!() };
    *max = Expr::AnnouncedX;
    assert!(cantrip::module(&face).is_err(), "the native table has constant ceilings");
}

#[test]
fn colored_cost_increases_keep_the_spell_filter_separate_from_the_mana_symbols() {
    let face = native_face("Derelor");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("CostDelta::more_mana("), "{generated}");
    assert!(generated.contains("CostDelta::more_mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0]"), "{generated}");
    assert!(generated.contains(".color(Color::Black)"));
    assert!(cantrip::planned::without_features(&["colored-spell-cost-increases"], || cantrip::module(&face)).is_err());
}

#[test]
fn one_graveyard_is_chosen_as_a_cost_not_as_a_target_or_resolving_effect() {
    let face = native_face("Night Soil");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("exile_from_a_graveyard_of(2,"));
    assert!(generated.contains(".is(CardType::Creature)"));
    assert!(!generated.contains("g.exile("));
    assert!(!generated.contains("targets: &[TargetSlot"));
    assert!(cantrip::planned::without_features(&["single-graveyard-exile-costs"], || cantrip::module(&face)).is_err());
}

#[test]
fn the_native_graveyard_cost_does_not_claim_self_exile_or_other_set_constraints() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Night Soil");
    face.abilities[0] = Ability::Scoped {
        zones: vec![EventZone::Graveyard], condition: None,
        ability: Box::new(face.abilities[0].clone()),
    };
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="single-graveyard cost that can exile its own ability source"));
    let mut face = native_face("Night Soil");
    let Ability::Activated { cost, .. } = &mut face.abilities[0] else { panic!() };
    let CostPart::Constrained { constraints, .. } = &mut cost.parts[1] else { panic!() };
    *constraints = vec![SelectionConstraint::DifferentNames];
    assert!(cantrip::module(&face).is_err());
}

#[test]
fn block_taxes_test_both_combatants_and_charge_the_declaring_player() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Hipparion");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("block_costs: &["));
    assert!(generated.contains("blocker == asker.source"));
    assert!(generated.contains("q.chars(attacker).power"));
    assert!(generated.contains("-> Option<ManaCost>"));
    assert!(cantrip::planned::without_features(&["mana-block-costs"], || cantrip::module(&face)).is_err());
    let Ability::Static { changes, .. } = &mut face.abilities[0] else { panic!() };
    let Change::Rule(RuleChange::CombatCost { cost, .. }) = &mut changes[0] else { panic!() };
    cost.parts = vec![CostPart::Life(Expr::Constant(1))];
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="nonmana cost to block"));
}

#[test]
fn restricted_retargeting_keeps_the_original_condition_and_uses_native_legality() {
    let face = native_face("Reflecting Mirror");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("g.change_target_where("));
    assert!(generated.contains("matches!(target, Target::Player(_))"));
    assert!(generated.contains("q.state().targets(i).iter().any("));
    assert!(generated.contains("matches!(t, Target::Player(p) if"));
    assert!(generated.contains(".x_is("));
    assert!(cantrip::planned::without_features(&["restricted-retargeting"], || cantrip::module(&face)).is_err());
}

#[test]
fn x_payment_reads_only_the_actual_mana_spent_on_this_spell_s_x() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Soul Burn");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("x_mana: XMana { types: 12, one_each: false }"));
    assert!(generated.contains("Event::SpentOnX { on, cast: true, mana, n, .. }"));
    assert!(generated.contains("on == item.object"));
    assert!(generated.contains("4u8 & (1u8 << mana as u8)"));
    assert!(!generated.contains("Event::ManaSpent"));
    for feature in ["x-mana-types", "spent-on-x-events"] {
        assert!(cantrip::planned::without_features(&[feature], || cantrip::module(&face)).is_err());
    }
    let Ability::Casting(CastingRule::RestrictMana { portion, .. }) = &mut face.abilities[0] else { panic!() };
    *portion = ManaCostPortion::All;
    assert!(cantrip::module(&face).is_err(), "the X interface does not restrict the rest of a cost");
}

#[test]
fn untap_replacement_counter_removal_belongs_to_the_same_simultaneous_event() {
    use cantrip_oracle_codegen::ir::*;
    let face = native_face("Freyalise's Winds");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("ReplacementKind::Untap"));
    assert!(generated.contains("u.untap_step == Some(u.controller)"));
    assert!(generated.contains("u.untaps = false"));
    assert!(generated.contains("u.remove[CounterKind::Wind as usize]"));
    assert!(!generated.contains("g.remove_counters"));
    assert!(cantrip::planned::without_features(&["untap-replacement-proposals"], || cantrip::module(&face)).is_err());
    let mut unsupported = face;
    let Ability::Replacement { body, .. } = &mut unsupported.abilities[1] else { panic!() };
    let Instruction::Effect { operation: Operation::RemoveCounters { removals }, .. } = &mut body.instructions[1] else { panic!() };
    removals[0].amount = CounterAmount::Count(Count::Exactly(Expr::Constant(1)));
    assert!(cantrip::gaps(&unsupported).iter().any(|gap|gap.feature=="untap replacement with instructions outside the native proposal"));
}

#[test]
fn spell_x_receipts_do_not_claim_snow_provenance_or_repeatable_activation_identity() {
    use cantrip_oracle_codegen::ir::*;
    let mut snow = native_face("Soul Burn");
    let Ability::Spell { body, .. } = &mut snow.abilities[1] else { panic!() };
    let Instruction::Effect { operation: Operation::GainLife { amount: Expr::Minimum(_, amount), .. }, .. } = &mut body.instructions[1] else { panic!() };
    let Expr::ManaOf { filter, .. } = &mut **amount else { panic!() };
    filter.snow = true;
    assert!(cantrip::gaps(&snow).iter().any(|gap|gap.feature=="numeric expression ManaOf"));
    let mut spell = native_face("Soul Burn");
    let Ability::Spell { announcement, body } = spell.abilities.remove(1) else { panic!() };
    let mut activation = native_face("Reflecting Mirror");
    let Ability::Activated { limits, .. } = &activation.abilities[0] else { panic!() };
    let mut limits = limits.clone();
    limits.modifiers.clear();
    activation.abilities = vec![Ability::Activated {
        origin: ActivationOrigin::Ordinary,
        cost: Cost { parts: vec![CostPart::Mana(vec![ManaSymbol::X])] },
        limits, announcement, body,
    }];
    assert!(cantrip::gaps(&activation).iter().any(|gap|gap.feature=="numeric expression ManaOf"));
    let mut delayed = native_face("Soul Burn");
    let Ability::Spell { body, .. } = &mut delayed.abilities[1] else { panic!() };
    body.instructions = vec![Instruction::Effect { result: ResultId(0), operation: Operation::CreateDelayedTrigger {
        event: Box::new(EventSpec::Turn { point: TurnPoint::EndStepBegins, players: PlayerSet::One(PlayerRef::AbilityController) }),
        start: DelayedStart::Immediately, expires: None, intervening_if: None,
        body: Program { instructions: vec![Instruction::Effect { result: ResultId(0), operation: Operation::GainLife {
            players: PlayerSet::One(PlayerRef::AbilityController),
            amount: Expr::ManaOf { mana: ManaOccurrence::SpentOnX, filter: SpentManaFilter { colors: None, snow: false } },
        } }] },
    } }];
    assert!(cantrip::gaps(&delayed).iter().any(|gap|gap.feature=="numeric expression ManaOf"));
}

#[test]
fn paid_damage_prevention_is_a_native_shield_and_keeps_the_original_damage_amount() {
    use cantrip_oracle_codegen::ir::*;
    for name in ["Errant Minion", "Power Leak"] {
        let face = native_face(name);
        let generated = cantrip::module(&face).unwrap();
        assert!(generated.contains("g.damage_preventing("));
        assert!(generated.contains("cantrip_mechanics::prevention::DAMAGE_DEALT_BY"));
        assert!(generated.contains("g.may_pay_times("));
        assert!(!generated.contains("g.may_pay_x("), "this payment has no X cost");
        assert!(!generated.contains("damage::partly_prevented"));
        assert!(cantrip::planned::without_features(&["damage-with-prevention"], || cantrip::module(&face)).is_err());
        assert!(cantrip::planned::without_features(&["repeated-mana-payments"], || cantrip::module(&face)).is_err());
    }
    let mut face = native_face("Errant Minion");
    let Ability::Triggered { body, .. } = &mut face.abilities[1] else { panic!() };
    let Instruction::Effect { operation: Operation::DealDamageWith { recipients, .. }, .. } = &mut body.instructions[1] else { panic!() };
    *recipients = Recipients::Players(PlayerSet::All);
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="partly prevented damage to multiple recipients"));
}

#[test]
fn regeneration_bans_remain_open_sets_and_scoped_replacements_use_the_native_variant() {
    let face = native_face("Runesword");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("LastingRuleKind::CantBeRegenerated"));
    assert!(generated.contains("EffectKind::ReplacementRule {"));
    assert!(!generated.contains("EffectKind::Replacement {"));
    assert!(cantrip::planned::without_features(&["lasting-rules"], || cantrip::module(&face)).is_err());
    let legacy = cantrip::planned::without_features(&["explicit-redirection-destinations"], || cantrip::module(&face)).unwrap();
    assert!(legacy.contains("EffectKind::Replacement {"));
}

#[test]
fn the_legacy_named_damage_rule_keeps_both_prevention_and_redirection_prohibitions() {
    let mut face = native_face("Whippoorwill");
    face.name = "Another bird".into();
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("LastingRuleKind::DamageCantBePrevented"));
    assert!(generated.contains("LastingRuleKind::DamageCantBeRedirected"));
    assert!(generated.contains("EffectKind::CantBeRegenerated") || generated.contains("LastingRuleKind::CantBeRegenerated"));
    assert!(generated.contains("let data = [new.0, u32::MAX]"));
    assert!(generated.contains("Some(ObjectId(item.data[0]))"), "move the card produced by the death event");
    assert!(cantrip::planned::without_features(&["lasting-rules"], || cantrip::module(&face)).is_err());
}

#[test]
fn every_native_interface_fixture_generates_a_real_card_definition() {
    let records = corpus::read_cards(&std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/native_interface_records.jsonl")).unwrap();
    for face in records.into_iter().flat_map(|record|record.card.faces) {
        let generated = cantrip::module(&face).unwrap_or_else(|error|panic!("{}: {error:?}",face.name));
        assert!(generated.contains("pub static CARD: CardDef ="), "{}", face.name);
    }
}

#[test]
fn optional_draw_step_skipping_uses_the_native_step_rule_and_runs_only_the_chosen_branch() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Fasting");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("step_begin: &[StepBeginRule"));
    assert!(generated.contains("skip your draw step?"));
    assert!(generated.contains(".no_default()"));
    assert!(generated.contains("g.gain_life("));
    assert!(!generated.contains("Proposal::BeginDrawStep"));
    assert!(cantrip::planned::without_features(&["draw-step-begin-replacements"], || cantrip::module(&face)).is_err());
    let Ability::Replacement { event: ReplacementEvent::Player { players, .. }, .. } = &mut face.abilities[1] else { panic!() };
    *players = PlayerSet::All;
    assert!(cantrip::module(&face).is_err(), "the native table asks only its controller");
}

#[test]
fn an_advance_payment_cancels_only_its_own_delayed_ability_before_its_deadline() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Nafs Asp");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("g.delayed_trigger_unless("));
    assert!(generated.contains(".special_action()"));
    assert!(generated.contains("Duration::UntilNextStepOf { step: Step::Draw"));
    assert!(generated.contains("PlayerId(item.data[1] as u8)"), "capture the damaged player");
    assert!(generated.contains("Target::Player(subject), payer"));
    assert!(generated.contains("item.subject.player()"), "the future body reads that captured player");
    assert!(cantrip::planned::without_features(&["cancelable-delayed-triggers"], || cantrip::module(&face)).is_err());
    let Ability::Triggered { body, .. } = &mut face.abilities[0] else { panic!() };
    let Instruction::Effect { operation: Operation::DelayedUnlessPaid { body, .. }, .. } = &mut body.instructions[0] else { panic!() };
    let Operand::Program(body) = &mut **body else { panic!() };
    let Instruction::Effect { operation: Operation::LoseLife { amount, .. }, .. } = &mut body.instructions[0] else { panic!() };
    *amount = Expr::AnnouncedX;
    assert!(cantrip::module(&face).is_err(), "the native player capture does not retain another activation's X");
}

#[test]
fn battlefield_put_replacements_keep_the_actor_and_wait_until_after_the_arrival() {
    use cantrip_oracle_codegen::ir::*;
    let mut face = native_face("Land Equilibrium");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("z.put_by.is_some_and(|actor|"));
    assert!(generated.contains(".afterward("));
    assert!(!generated.contains("z.after("));
    assert!(generated.contains("z.put_by.is_none()"));
    for feature in ["post-move-replacements", "putting-player-replacements"] {
        assert!(cantrip::planned::without_features(&[feature], || cantrip::module(&face)).is_err());
    }
    let Ability::Replacement { body, .. } = &mut face.abilities[0] else { panic!() };
    let Instruction::Effect { operation: Operation::ModifyEvent { change: EventChange::Then(body) }, .. } = &mut body.instructions[0] else { panic!() };
    let Instruction::Effect { operation: Operation::Sacrifice { objects, .. }, .. } = &mut body.instructions[0] else { panic!() };
    *objects = ObjectSet::One(ObjectRef::EventSubject);
    assert!(cantrip::gaps(&face).iter().any(|gap|gap.feature=="post-move replacement reading the moved object's new identity"));
}

#[test]
fn object_specific_entry_replacements_end_with_their_object_instead_of_becoming_global_rules() {
    let face = native_face("Semester's End");
    let generated = cantrip::module(&face).unwrap();
    assert!(generated.contains("EffectKind::Replacement { card: item.ability.card, index: 1, to: Target::None, into: Target::None }"));
}
