// Kami of the Palace Fields
// status: planned
// requires: engine: no Soulshift
// Flying, first strike
// Soulshift 5 (When this creature dies, you may return target Spirit card with mana value 5 or less from your graveyard to your hand.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-soulshift" => abilities: cantrip_mechanics::soulshift::trigger::<5>("Soulshift 5"));

pub static CARD: CardDef = CardDef { name: "Kami of the Palace Fields", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
