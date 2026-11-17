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
