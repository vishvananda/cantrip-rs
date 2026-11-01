// Inkwell Leviathan
// status: ready
// Trample
// Islandwalk (This creature can't be blocked as long as defending player controls an Island.)
// Shroud (This creature can't be the target of spells or abilities.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Inkwell Leviathan", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Leviathan]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(11), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Islandwalk,Keyword::Shroud]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
