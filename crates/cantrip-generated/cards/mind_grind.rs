// Mind Grind
// status: planned
// requires: operation LimitX
// Each opponent reveals cards from the top of their library until they reveal X land cards, then puts all cards revealed this way into their graveyard. X can't be 0.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-limit-x" => cantrip_mechanics::announce::limit_x(g, item, Comparison::Greater, { let q: &dyn Query = g; 0i64 }));
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_1.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_1, bound_9000) in players_1.iter().copied().enumerate() { let (carried_4_0, carried_4_1,) = if let Some(present_4_0) = { let q: &dyn Query = g; Some(bound_9000) } { let owner_3 = { let q: &dyn Query = g; present_4_0 }; let count_3 = { let q: &dyn Query = g; (i64::from(item.x)) as usize }; let (found_3, rest_3) = cantrip_mechanics::library::reveal_until_n(g, owner_3, count_3, &|q: &dyn Query, id: ObjectId| q.chars(id).is(CardType::Land)); let shown_3 = found_3.len() + rest_3.len(); let objects_3: Vec<ObjectId> = g.state().zone(Zone::library(owner_3)).iter().copied().take(shown_3).collect();
(objects_3.clone(), found_3.clone(),) } else { Default::default() };
 each_5[index_1] = carried_4_0.clone(); all_5.extend(each_5[index_1].iter().copied());
each_6[index_1] = carried_4_1.clone(); all_6.extend(each_6[index_1].iter().copied());
   } 
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_7.sort_by_key(|p| order.iter().position(|o| o == p)); }
let objects_8 = { let q: &dyn Query = g; ({ let mut all: Vec<ObjectId> = q.state().battlefield().to_vec(); all.extend_from_slice(q.state().zone(Zone::STACK)); all.extend_from_slice(q.state().zone(Zone::COMMAND)); for p in q.state().seats() { for zone in [Zone::library(p), Zone::hand(p), Zone::graveyard(p), Zone::exile(p)] { all.extend_from_slice(q.state().zone(zone)); } } all }).into_iter().filter(|&id| (all_5.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let moved_8 = { let moves: Vec<(ObjectId, Zone)> = objects_8.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mind Grind", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
