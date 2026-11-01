// Master of the Wild Hunt Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: operation ChooseAtRandom
// {2}{G}: Create a green creature token that's a 2/2 Wolf, a 2/3 Antelope with forestwalk, a 3/2 Cat with shroud, or a 4/4 Rhino with trample, chosen at random.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Wolf).name, token_words: token!(Wolf).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Wolf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_6 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Antelope).name, token_words: token!(Antelope).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Antelope]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Forestwalk]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_8 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Cat).name, token_words: token!(Cat).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Shroud]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_10 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rhino).name, token_words: token!(Rhino).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rhino]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-at-random" => cantrip_mechanics::choose::at_random(g, asker.source, ChoiceKind::Action, &[&[|g: &mut dyn Game| -> Result<(), Suspend> { let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = g.words(asker.source);
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
hoisted_4.extend(created_2.clone());
 Ok(()) }], &[|g: &mut dyn Game| -> Result<(), Suspend> { let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_5 = Vec::new();
let count_5_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_5_0 = g.card_id(&token_6::CARD);
for player in players_5 {
tokens_5.extend(std::iter::repeat((def_5_0,player)).take(count_5_0 as usize));
} let words_5 = g.words(asker.source);
let created_5 = g.create_tokens_worded(&tokens_5, false, None, words_5)?;
g.end_sentence();
 Ok(()) }], &[|g: &mut dyn Game| -> Result<(), Suspend> { let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_7 = Vec::new();
let count_7_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_7_0 = g.card_id(&token_8::CARD);
for player in players_7 {
tokens_7.extend(std::iter::repeat((def_7_0,player)).take(count_7_0 as usize));
} let words_7 = g.words(asker.source);
let created_7 = g.create_tokens_worded(&tokens_7, false, None, words_7)?;
g.end_sentence();
 Ok(()) }], &[|g: &mut dyn Game| -> Result<(), Suspend> { let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_9 = Vec::new();
let count_9_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_9_0 = g.card_id(&token_10::CARD);
for player in players_9 {
tokens_9.extend(std::iter::repeat((def_9_0,player)).take(count_9_0 as usize));
} let words_9 = g.words(asker.source);
let created_9 = g.create_tokens_worded(&tokens_9, false, None, words_9)?;
g.end_sentence();
 Ok(()) }]])?);
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: 3 }));

pub static CARD: CardDef = CardDef { name: "Master of the Wild Hunt Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{2}{G}: Create a green creature token that's a 2/2 Wolf, a 2/3 Antelope with forestwalk, a 3/2 Cat with shroud, or a 4/4 Rhino with trample, chosen at random.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD,&token_6::CARD,&token_8::CARD,&token_10::CARD];
