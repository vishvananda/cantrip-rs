// Battle for Bretagard
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — Create a 1/1 white Human Warrior creature token.
// II — Create a 1/1 green Elf Warrior creature token.
// III — Choose any number of artifact tokens and/or creature tokens you control with different names. For each of them, create a token that's a copy of it.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Human Warrior).name, token_words: token!(Human Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Elf Warrior).name, token_words: token!(Elf Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (((q.chars(id).is(CardType::Artifact) && q.state().obj(id).token) || (q.chars(id).is(CardType::Creature) && q.state().obj(id).token)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let picked_4 = cantrip_mechanics::choose::select(g, player_4, asker.source, Purpose::ChooseAsEnters, pool_4, cantrip_mechanics::select::Selection::of(count_4, &[]).with(cantrip_mechanics::select::DIFFERENT_NAMES))?;
g.end_sentence();
let originals_5 = { let q: &dyn Query = g; picked_4.clone() }; let player_5 = { let q: &dyn Query = g; Some(asker.controller) }; let created_5 = if let Some(player) = player_5 { let copies: Vec<(Copiable, PlayerId)> = originals_5.iter().map(|&original| (g.copiable_values(original), player)).collect(); g.create_token_copies_together(&copies)? } else { Vec::new() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Battle for Bretagard", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Create a 1/1 white Human Warrior creature token.").targets(&[]),chapter(2, ability_0_1, "II — Create a 1/1 green Elf Warrior creature token.").targets(&[]),chapter(3, ability_0_2, "III — Choose any number of artifact tokens and/or creature tokens you control with different names. For each of them, create a token that's a copy of it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD,&token_3::CARD];
