// Fluctuator
// status: planned
// requires: a keyword's cost changed
// Cycling abilities you activate cost {2} less to activate.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Cycling, applies: |q, asker, player, card| player == asker.controller && true, change: KeywordCostChange::Reduce(Cost::generic(2)), per: |q, asker| 1, first_each_turn: false });

pub static CARD: CardDef = CardDef { name: "Fluctuator", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
