// Turtles in Time
// status: ready
// Return all creatures to their owners' hands. Each player may shuffle their hand and graveyard into their library, then each player who does draws seven cards.
// Exile Turtles in Time.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_1_0 = vec![false; players_1.len()];
let mut gathered_2: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_1_0[index_1] = true;
gathered_2.extend({ let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } });
   }
 } 
let objects_4 = ({ let q: &dyn Query = g; gathered_2.clone() }).into_iter().rev().collect::<Vec<ObjectId>>(); let moved_4 = g.put_into_library(&objects_4, LibraryPlace::Top);
g.end_sentence();
let mut each_7 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if guard_1_0[index_1] {
for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
let happened_6: bool = true;
 each_7[index_1] = happened_6;
  }
 } 
g.end_sentence();
let amount_8 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_8 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_1.iter().position(|&next| next == p)).is_some_and(|at| (each_7[at] as i64) != 0)).collect::<Vec<PlayerId>>() };
for player in players_8 { g.draw(player, amount_8); }
g.end_sentence();
let objects_9 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_9 = g.exile_all(&objects_9);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Turtles in Time", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
