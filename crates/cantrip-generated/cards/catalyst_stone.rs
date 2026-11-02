// Catalyst Stone
// status: planned
// requires: a keyword's cost changed
// Flashback costs you pay cost {2} less.
// Flashback costs your opponents pay cost {2} more.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_0_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player == asker.controller && true });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_0_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_1_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player != asker.controller && true });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_1_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Flashback, applies: ability_0_0_keyword_cards_0, change: KeywordCostChange::Reduce(Cost::generic(2)), per: ability_0_0_keyword_per_0, first_each_turn: false });
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Flashback, applies: ability_1_0_keyword_cards_0, change: KeywordCostChange::Increase(Cost::generic(2)), per: ability_1_0_keyword_per_0, first_each_turn: false });

pub static CARD: CardDef = CardDef { name: "Catalyst Stone", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
