// Myrkul's Edict
// status: ready
// Roll a d20.
// 1—9 | Choose an opponent. That player sacrifices a creature of their choice.
// 10—19 | Each opponent sacrifices a creature of their choice.
// 20 | Each opponent sacrifices a creature with the greatest power among creatures that player controls.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0) >= (1i64) && (value_0) <= (9i64)) } {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_1) = cantrip_mechanics::choose::player(g, player_1, asker.source, candidates_1, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(chosen_1).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_2 { let eligible: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_2.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_2.clone(), &[]))? }; for object in taken { picks_2.push((player, object)); } }
let sacrificed_2 = g.sacrifice_each(&picks_2);
g.end_sentence();
hoisted_3.extend(sacrificed_2.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
if { let q: &dyn Query = g; ((value_0) >= (10i64) && (value_0) <= (19i64)) } {
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_5 { let eligible: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_5.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_5.clone(), &[]))? }; for object in taken { picks_5.push((player, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
hoisted_6.extend(sacrificed_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
if { let q: &dyn Query = g; (value_0) == (20i64) } {
let mut players_8: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut gathered_9: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { let objects_10 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) == ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).max().unwrap_or(0))))).collect::<Vec<ObjectId>>() }; let players_10 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_10: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_10 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_10 { let eligible: Vec<ObjectId> = objects_10.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_10.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_10.clone(), &[]))? }; for object in taken { picks_10.push((player, object)); } }
gathered_9.extend(picks_10.iter().copied());
    } 
let sacrificed_9 = g.sacrifice_each(&gathered_9);
g.end_sentence();
hoisted_11.extend(sacrificed_9.clone());
hoisted_12.extend({ let q: &dyn Query = g; gathered_9.iter().map(|pick| pick.1).collect::<Vec<ObjectId>>().iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Myrkul's Edict", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
