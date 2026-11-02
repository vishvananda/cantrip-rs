// Plague Boiler
// status: ready
// At the beginning of your upkeep, put a plague counter on this artifact.
// {1}{B}{G}: Put a plague counter on this artifact or remove a plague counter from it.
// When this artifact has three or more plague counters on it, sacrifice it. If you do, destroy all nonland permanents.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Plague, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let offered_1: Vec<u8> = [(0u8, true), (1u8, { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| q.state().obj(o).counters(CounterKind::Plague) > 0) })].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_1 = item.ability.card; let picked_1 = if offered_1.len() > 1 { offered_1[cantrip_mechanics::choose::one_of(g, decider_1, asker.source, offered_1.iter().map(|&option| Label::Card(card_1, option)).collect())? as usize] } else { offered_1.first().copied().unwrap_or(u8::MAX) }; match picked_1 {
0 => {
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::Plague, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_3.extend(applied_2.clone());
}
1 => {
let mut removed_4: i64 = 0;
let objects_4_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_4_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_4_0 { removed_4 += i64::from(g.remove_counters(object, CounterKind::Plague, amount_4_0)); } });
g.end_sentence();
}
_ => {}
}
g.end_sentence();
 Ok(()) }
fn ability_2_state(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Plague))) >= (3i64)) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
let paid_5 = !picks_5.is_empty();
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
let objects_6 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let destroyed_6 = g.destroy_all(&objects_6);
g.end_sentence();
hoisted_7.extend(destroyed_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Plague Boiler", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, put a plague counter on this artifact.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{1}{B}{G}: Put a plague counter on this artifact or remove a plague counter from it.").targets(&[]),state_triggered(ability_2_state, ability_2, "When this artifact has three or more plague counters on it, sacrifice it. If you do, destroy all nonland permanents.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Put a plague counter on this artifact", "Remove a plague counter from it"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
