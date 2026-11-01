// Collision of Realms
// status: ready
// Each player shuffles all creatures they own into their library. Each player who shuffled a nontoken creature into their library this way reveals cards from the top of their library until they reveal a creature card, then puts that card onto the battlefield and the rest on the bottom of their library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() });
    } 
let objects_3 = { let q: &dyn Query = g; gathered_1.clone() }; let moved_3 = g.put_into_library(&objects_3, LibraryPlace::ShuffledIn);
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| ((((objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| Some(q.state().obj(o).owner) == Some(bound_0)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let owner_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let count_5 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_5, rest_5) = cantrip_mechanics::library::reveal_until_n(g, owner_5, count_5, &|q: &dyn Query, id: ObjectId| q.chars(id).is(CardType::Creature)); let shown_5 = found_5.len() + rest_5.len(); let objects_5: Vec<ObjectId> = g.state().zone(Zone::library(owner_5)).iter().copied().take(shown_5).collect();
 each_6[index_4] = objects_5.clone(); all_6.extend(each_6[index_4].iter().copied());
each_7[index_4] = found_5.clone(); all_7.extend(each_7[index_4].iter().copied());
   } 
g.end_sentence();
let objects_8 = ({ let q: &dyn Query = g; (all_6.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_8 = { let rows_8: Vec<(ObjectId, PlayerId, bool)> = objects_8.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_8 = g.put_all_onto_battlefield_as(&rows_8)?; came_8.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let library_9 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let cards_9: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (each_6[index_4].clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_9)).collect(); let placed_9 = cards_9.clone();
g.bottom_in_random_order(library_9, &cards_9);
 each_10[index_4] = placed_9.clone(); all_10.extend(each_10[index_4].iter().copied());
each_11[index_4] = { let q: &dyn Query = g; placed_9.clone() }; all_11.extend(each_11[index_4].iter().copied());
   } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Collision of Realms", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
