// Midnight Crusader Shuttle
// status: planned
// requires: earlier objects GainedControl
// requires: instruction ChooseOne
// requires: operation ChangeCombat
// requires: villainous choice
// Midnight Entity — Whenever this Vehicle attacks, defending player faces a villainous choice — That player sacrifices a creature of their choice, or you gain control of a creature of your choice that player controls until end of turn. If you gain control of a creature this way, tap it, and it's attacking that player.
// Crew 2
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if let Some(present_2_0) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { if let Some(present_3_1) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { planned!("villainous-choice" => cantrip_mechanics::choose::villainous(g, { let q: &dyn Query = g; present_2_0 }, asker.source, &["", " Change characteristics until end of turn. Tap the chosen permanents. "])?);
let decider_2 = { let q: &dyn Query = g; present_3_1 }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_2, asker.source, vec![Label::Card(card_2, 0), Label::Card(card_2, 1)])?) {
0 => {
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_3 { let eligible: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_3.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_3.clone(), &[]))? }; for object in taken { picks_3.push((player, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
hoisted_4.extend(sacrificed_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
_ => {
let (carried_8_0, carried_8_1, carried_8_2,) = if let Some(present_8_0) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { let selected_7_player = { let q: &dyn Query = g; present_8_0 }; let selected_7_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_7_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_7 = { cantrip_mechanics::choose::select(g, selected_7_player, asker.source, Purpose::ChooseAsEnters, selected_7_candidates, cantrip_mechanics::select::Selection::of(selected_7_count, &[]))? };
(selected_7.clone(), hoisted_4.clone(), { let q: &dyn Query = g; hoisted_5.clone() },) } else { Default::default() };
let (carried_11_0,) = if let Some(present_11_0) = { let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) } { let objects_10 = { let q: &dyn Query = g; (carried_8_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let duration_10 = Duration::EndOfTurn;
let kind_10_0_0 = { let q: &dyn Query = g; EffectKind::Control(present_11_0) };
if let Some(source) = source_10 { cantrip_api::instruction(g, |g| { for &object in &objects_10 { g.add_effect(source, object, kind_10_0_0, duration_10);  } }); }
(objects_10.clone(),) } else { Default::default() };
g.end_sentence();
let objects_12 = { let q: &dyn Query = g; ((planned_value!("earlier objects GainedControl", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::GainedControl))).first().copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_12: Vec<ObjectId> = objects_12.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_12);
g.end_sentence();
planned!("operation-change-combat" => cantrip_mechanics::combat::change(g, asker, CombatChange::AssignAttack { objects: { let q: &dyn Query = g; (planned_value!("earlier objects GainedControl", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::GainedControl))).first().copied().into_iter().collect::<Vec<ObjectId>>() }, to: AttackAssignment::Player({ let q: &dyn Query = g; item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) }) })?);
g.end_sentence();
hoisted_14.extend(objects_12.clone());
hoisted_15.extend(carried_11_0.clone());
}
}
 } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Midnight Crusader Shuttle", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Midnight Entity — Whenever this Vehicle attacks, defending player faces a villainous choice — That player sacrifices a creature of their choice, or you gain control of a creature of your choice that player controls until end of turn. If you gain control of a creature this way, tap it, and it's attacking that player.").targets(&[]),cantrip_mechanics::crew::crew(2, "Crew 2")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
