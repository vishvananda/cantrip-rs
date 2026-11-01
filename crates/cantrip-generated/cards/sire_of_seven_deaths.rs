// Sire of Seven Deaths
// status: ready
// Reach, first strike
// Vigilance, menace
// Trample, lifelink
// Ward—Pay 7 life.
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Sire of Seven Deaths", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Reach,Keyword::FirstStrike,Keyword::Vigilance,Keyword::Menace,Keyword::Trample,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward_life::<7>("Ward--Pay 7 life")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
