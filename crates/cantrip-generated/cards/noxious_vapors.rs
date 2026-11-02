// Noxious Vapors
// status: ready
// Each player reveals their hand, chooses one card of each color from it, then discards all other nonland cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_1 = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
 each_2[index_0] = objects_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let player_4 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_4 = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut picked_4: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_4.iter().copied().filter(|id| !picked_4.contains(id)).filter(|&id| q.chars(id).colors.has(q.words(asker.source).color(Color::White))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_4.contains(&o) { picked_4.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_4.iter().copied().filter(|id| !picked_4.contains(id)).filter(|&id| q.chars(id).colors.has(q.words(asker.source).color(Color::Blue))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_4.contains(&o) { picked_4.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_4.iter().copied().filter(|id| !picked_4.contains(id)).filter(|&id| q.chars(id).colors.has(q.words(asker.source).color(Color::Black))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_4.contains(&o) { picked_4.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_4.iter().copied().filter(|id| !picked_4.contains(id)).filter(|&id| q.chars(id).colors.has(q.words(asker.source).color(Color::Red))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_4.contains(&o) { picked_4.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_4.iter().copied().filter(|id| !picked_4.contains(id)).filter(|&id| q.chars(id).colors.has(q.words(asker.source).color(Color::Green))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_4.contains(&o) { picked_4.push(o); } } }
 each_5[index_3] = picked_4.clone(); all_5.extend(each_5[index_3].iter().copied());
   } 
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_7: Vec<ObjectId> = Vec::new();
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { gathered_7.extend({ let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (({ let chosen: Vec<ObjectId> = all_5.clone(); (players_3.iter().copied().flat_map(|bound_0: PlayerId| ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&card_in_zone| true && true).collect::<Vec<ObjectId>>()).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() }).contains(&id) && !(q.chars(id).is(CardType::Land)))).collect::<Vec<ObjectId>>() });
    } 
let cards_9 = { let q: &dyn Query = g; gathered_7.clone() }; let discarded_9 = g.discard_together(&cards_9);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Noxious Vapors", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
