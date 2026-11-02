// Rampage of the Clans
// status: ready
// Destroy all artifacts and enchantments. For each permanent destroyed this way, its controller creates a 3/3 green Centaur creature token.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_4 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Centaur).name, token_words: token!(Centaur).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Centaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_2: Vec<(CardId, PlayerId)> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let def_3 = g.card_id(&token_4::CARD); let count_3 = { let q: &dyn Query = g; (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let player_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; gathered_2.extend(std::iter::repeat((def_3, player_3)).take(count_3 as usize));
    } 
let words_2 = g.words(asker.source); let created_2 = g.create_tokens_worded(&gathered_2, false, None, words_2)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rampage of the Clans", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_4::CARD];
