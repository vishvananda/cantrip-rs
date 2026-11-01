// Grenzo's Rebuttal
// status: ready
// Create a 4/4 red Ogre creature token. Starting with you, each player chooses an artifact, a creature, and a land from among the permanents controlled by the player to their left. Destroy each permanent chosen this way.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Ogre).name, token_words: token!(Ogre).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ogre]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(8)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_2 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_2.iter().position(|&p| Some(p) == first_2) { players_2.rotate_left(at); }
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let player_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(bound_0).and_then(|p: PlayerId| q.state().turn_order_from(p).filter(|&next| next != p).next()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut picked_3: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_3.iter().copied().filter(|id| !picked_3.contains(id)).filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_3, asker.source, Purpose::Destroy, open, 1, false)?; for o in one { if !picked_3.contains(&o) { picked_3.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_3.iter().copied().filter(|id| !picked_3.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_3, asker.source, Purpose::Destroy, open, 1, false)?; for o in one { if !picked_3.contains(&o) { picked_3.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_3.iter().copied().filter(|id| !picked_3.contains(id)).filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_3, asker.source, Purpose::Destroy, open, 1, false)?; for o in one { if !picked_3.contains(&o) { picked_3.push(o); } } }
 each_4[index_2] = picked_3.clone(); all_4.extend(each_4[index_2].iter().copied());
   } 
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; all_4.clone() }; let destroyed_5 = g.destroy_all(&objects_5);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Grenzo's Rebuttal", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
