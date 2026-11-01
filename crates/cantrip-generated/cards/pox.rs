// Pox
// status: ready
// Each player loses a third of their life, then discards a third of the cards in their hand, then sacrifices a third of the creatures they control of their choice, then sacrifices a third of the lands they control of their choice. Round up each time.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let amount_1 = { let q: &dyn Query = g; ((Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))).saturating_add(2).div_euclid(3)).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
    } 
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_3: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let amount_4 = { let q: &dyn Query = g; (((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).saturating_add(2).div_euclid(3)).max(0) as u32 }; gathered_3.extend(cantrip_mechanics::choose_from_hand(g, bound_0, asker.source, amount_4)?);
    } 
let cards_5 = { let q: &dyn Query = g; gathered_3.clone() }; let discarded_5 = g.discard_together(&cards_5);
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_7: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { let objects_8 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_8 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_8: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_8 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).saturating_add(2).div_euclid(3)).max(0) as u32) }; for player in players_8 { let eligible: Vec<ObjectId> = objects_8.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_8.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_8.clone(), &[]))? }; for object in taken { picks_8.push((player, object)); } }
gathered_7.extend(picks_8.iter().copied());
    } 
let sacrificed_7 = g.sacrifice_each(&gathered_7);
g.end_sentence();
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_10: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let objects_11 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let players_11 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_11: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_11 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).saturating_add(2).div_euclid(3)).max(0) as u32) }; for player in players_11 { let eligible: Vec<ObjectId> = objects_11.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_11.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_11.clone(), &[]))? }; for object in taken { picks_11.push((player, object)); } }
gathered_10.extend(picks_11.iter().copied());
    } 
let sacrificed_10 = g.sacrifice_each(&gathered_10);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Pox", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
