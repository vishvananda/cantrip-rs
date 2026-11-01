// Rograkh, Son of Rohgahh
// status: ready
// First strike, menace, trample
// Partner (You can have two commanders if both have partner.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Rograkh, Son of Rohgahh", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kobold,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(8)), power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike,Keyword::Menace,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
