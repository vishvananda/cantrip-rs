// All Hallow's Eve
// status: ready
// Exile All Hallow's Eve with two scream counters on it.
// At the beginning of your upkeep, if this card is exiled with a scream counter on it, remove a scream counter from it. If there are no more scream counters on it, put it into your graveyard and each player returns all creature cards from their graveyard to the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = { let counters: Vec<(CounterKind, i16)> = vec![(CounterKind::Scream, { let q: &dyn Query = g; (2i64).max(0) as i16 })]; let exiled = g.exile_all(&objects_0); let puts: Vec<(ObjectId, CounterKind, i16)> = exiled.iter().flat_map(|&o| counters.iter().map(move |&(kind, n)| (o, kind, n))).collect(); g.add_counters_together(&puts); exiled };
g.end_sentence();
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Scream))) > (0i64)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_1_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += i64::from(g.remove_counters(object, CounterKind::Scream, amount_1_0)); } });
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Scream))) == (0i64)) } {
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_2 = { let moves: Vec<(ObjectId, Zone)> = objects_2.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { gathered_4.extend({ let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() });
    } 
let objects_6 = ({ let q: &dyn Query = g; gathered_4.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_6 = { let rows_6: Vec<(ObjectId, PlayerId, bool)> = objects_6.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_6 = g.put_all_onto_battlefield_as(&rows_6)?; came_6.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_7.extend(moved_2.clone());
hoisted_8.extend(moved_6.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_10.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "All Hallow's Eve", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, if this card is exiled with a scream counter on it, remove a scream counter from it. If there are no more scream counters on it, put it into your graveyard and each player returns all creature cards from their graveyard to the battlefield.").targets(&[]).condition(ability_1_if).functions_in(ZoneKinds::of(&[ZoneKind::Exile]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
