// Jaheira, Harper Emissary
// status: planned
// requires: engine: no Specialize
// Specialize {2}
// Hexproof from artifacts and enchantments
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-specialize" => abilities: cantrip_mechanics::specialize::ability(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Specialize {2}"));

pub static CARD: CardDef = CardDef { name: "Jaheira, Harper Emissary", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Elf,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, hexproof_from: HexproofFrom::NONE.and(HexproofFrom::card_type(CardType::Artifact)).and(HexproofFrom::card_type(CardType::Enchantment)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
