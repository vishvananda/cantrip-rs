// Rhox Maulers
// status: planned
// requires: engine: no Renown
// Trample (This creature can deal excess combat damage to the player or planeswalker it's attacking.)
// Renown 2 (When this creature deals combat damage to a player, if it isn't renowned, put two +1/+1 counters on it and it becomes renowned.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-renown" => abilities: cantrip_mechanics::renown::trigger::<2>("Renown 2"));

pub static CARD: CardDef = CardDef { name: "Rhox Maulers", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rhino,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
