// Tyrranax Rex
// status: ready
// This spell can't be countered.
// Trample, ward {4}, haste
// Toxic 4 (Players dealt combat damage by this creature also get four poison counters.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Tyrranax Rex", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered,Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<4>("Ward {4}")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, toxic: 4, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
