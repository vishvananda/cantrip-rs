// Guided Passage
// status: ready
// Reveal the cards in your library. An opponent chooses from among them a creature card, a land card, and a noncreature, nonland card. You put the chosen cards into your hand. Then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let player_2 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let pool_2 = { let q: &dyn Query = g; (objects_0.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let mut picked_2: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_2.iter().copied().filter(|id| !picked_2.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_2, asker.source, Purpose::PutIntoHand, open, 1, false)?; for o in one { if !picked_2.contains(&o) { picked_2.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_2.iter().copied().filter(|id| !picked_2.contains(id)).filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_2, asker.source, Purpose::PutIntoHand, open, 1, false)?; for o in one { if !picked_2.contains(&o) { picked_2.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_2.iter().copied().filter(|id| !picked_2.contains(id)).filter(|&id| (!(q.chars(id).is(CardType::Creature)) && !(q.chars(id).is(CardType::Land)))).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_2, asker.source, Purpose::PutIntoHand, open, 1, false)?; for o in one { if !picked_2.contains(&o) { picked_2.push(o); } } }

let objects_3 = { let q: &dyn Query = g; picked_2.clone() }; let moved_3 = g.return_to_hand_revealed(&objects_3);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Guided Passage", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
