// Warp World
// status: ready
// Each player shuffles all permanents they own into their library, then reveals that many cards from the top of their library. Each player puts all artifact, creature, and land cards revealed this way onto the battlefield, then does the same for enchantment cards, then puts all cards revealed this way that weren't put onto the battlefield on the bottom of their library.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() });
    } 
let objects_3 = { let q: &dyn Query = g; gathered_1.clone() }; let moved_3 = g.put_into_library(&objects_3, LibraryPlace::ShuffledIn);
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let objects_5 = { let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((((objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.state().obj(id).owner)).collect::<Vec<ObjectId>>()).len() as i64) as usize).collect::<Vec<ObjectId>>() }; let mut owners_5: Vec<PlayerId> = objects_5.iter().map(|o| g.state().obj(*o).owner).collect(); owners_5.dedup(); for owner in owners_5 { let cards: Vec<ObjectId> = objects_5.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
 each_6[index_4] = objects_5.clone(); all_6.extend(each_6[index_4].iter().copied());
   } 
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_8: Vec<ObjectId> = Vec::new();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { gathered_8.extend({ let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Land)) && ((all_6.clone()).contains(&id) && Some(bound_0) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() });
    } 
let objects_10 = ({ let q: &dyn Query = g; gathered_8.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_10 = { let rows_10: Vec<(ObjectId, PlayerId, bool)> = objects_10.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_10 = g.put_all_onto_battlefield_as(&rows_10)?; came_10.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let mut players_11: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_12: Vec<ObjectId> = Vec::new();
 for (index_11, bound_0) in players_11.iter().copied().enumerate() { gathered_12.extend({ let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Enchantment) && ((all_6.clone()).contains(&id) && Some(bound_0) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() });
    } 
let objects_14 = ({ let q: &dyn Query = g; gathered_12.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_14 = { let rows_14: Vec<(ObjectId, PlayerId, bool)> = objects_14.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_14 = g.put_all_onto_battlefield_as(&rows_14)?; came_14.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let mut players_15: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_16: Vec<ObjectId> = Vec::new();
 for (index_15, bound_0) in players_15.iter().copied().enumerate() { gathered_16.extend({ let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((all_6.clone()).contains(&id) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() });
    } 
let objects_18 = { let q: &dyn Query = g; gathered_16.clone() }; let moved_18 = { let order_18 = match objects_18.iter().all(|&o| g.state().obj(o).owner == asker.controller) { true => cantrip_mechanics::library::ask_library_order(g, asker.controller, asker.source, &objects_18, DestinationKind::LibraryBottom)?, false => objects_18.clone() }; g.put_into_library(&order_18, LibraryPlace::Bottom) };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Warp World", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 3, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
