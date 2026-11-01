// Debtors' Transport
// status: planned
// requires: engine: no Afterlife
// Afterlife 2 (When this creature dies, create two 1/1 white and black Spirit creature tokens with flying.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-afterlife" => abilities: cantrip_mechanics::afterlife::trigger::<2>("Afterlife 2"));
planned_item!("engine-no-afterlife" => extras: cantrip_mechanics::afterlife::SPIRIT);

pub static CARD: CardDef = CardDef { name: "Debtors' Transport", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Thrull]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
