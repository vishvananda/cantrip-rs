// Kitsune Blademaster
// status: planned
// requires: engine: no Bushido
// First strike
// Bushido 1 (Whenever this creature blocks or becomes blocked, it gets +1/+1 until end of turn.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-bushido" => abilities: cantrip_mechanics::bushido::trigger::<1>("Bushido 1"));

pub static CARD: CardDef = CardDef { name: "Kitsune Blademaster", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fox,Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
