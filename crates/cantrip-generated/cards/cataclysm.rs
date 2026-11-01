// Cataclysm
// status: ready
// Each player chooses from among the permanents they control an artifact, a creature, an enchantment, and a land, then sacrifices the rest.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut picked_1: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::Keep, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
 each_2[index_0] = picked_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; { let chosen: Vec<ObjectId> = all_2.clone(); (players_0.iter().copied().flat_map(|bound_0: PlayerId| q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() } }; let players_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Cataclysm", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
