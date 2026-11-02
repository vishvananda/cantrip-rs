// Invoke the Ancients
// status: ready
// Create two 4/5 green Spirit creature tokens. For each of them, put your choice of a reach counter, a vigilance counter, or a trample counter on it.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Spirit).name, token_words: token!(Spirit).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; created_0.clone() }; let mut rows_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for object in objects_2 { let picked = cantrip_mechanics::choose::option(g, asker.controller, asker.source, item.ability.card, 0..3)? - 0; rows_2.push((object, [CounterKind::Reach, CounterKind::Vigilance, CounterKind::Trample][picked as usize], 1)); } g.add_counters_together(&rows_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Invoke the Ancients", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 4, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["reach counter", "vigilance counter", "trample counter"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
