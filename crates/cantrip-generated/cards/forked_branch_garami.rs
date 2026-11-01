// Forked-Branch Garami
// status: planned
// requires: engine: no Soulshift
// Soulshift 4, soulshift 4 (When this creature dies, you may return up to two target Spirit cards with mana value 4 or less from your graveyard to your hand.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-soulshift" => abilities: cantrip_mechanics::soulshift::trigger::<4>("Soulshift 4"));
planned_item!("engine-no-soulshift" => abilities: cantrip_mechanics::soulshift::trigger::<4>("Soulshift 4"));

pub static CARD: CardDef = CardDef { name: "Forked-Branch Garami", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
