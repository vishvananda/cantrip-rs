// Gnat Miser
// status: planned
// requires: a maximum hand size of other players
// Each opponent's maximum hand size is reduced by one.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-maximum-hand-size-of-other-players" => hand_size_rules: HandSizeRule { applies: |q, asker, player| player != asker.controller, size: HandSize::Adjust(-1) });

pub static CARD: CardDef = CardDef { name: "Gnat Miser", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rat,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
