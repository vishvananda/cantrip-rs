// Petrified Wood-Kin
// status: planned
// requires: engine: no BloodthirstX
// This spell can't be countered.
// Bloodthirst X (This creature enters with X +1/+1 counters on it, where X is the damage dealt to your opponents this turn.)
// Protection from instants
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-bloodthirst-x" => enters: cantrip_mechanics::bloodthirst::enters_x);

pub static CARD: CardDef = CardDef { name: "Petrified Wood-Kin", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::protection::from_type::<{ CardType::Instant as u8 }>("Protection from Instant")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
