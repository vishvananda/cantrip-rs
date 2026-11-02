// Exiled Doomsayer
// status: planned
// requires: a keyword's cost changed
// All morph costs cost {2} more. (This doesn't affect the cost to cast creature spells face down.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_0_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { true && true });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_0_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Morph, applies: ability_0_0_keyword_cards_0, change: KeywordCostChange::Increase(Cost::generic(2)), per: ability_0_0_keyword_per_0, first_each_turn: false });

pub static CARD: CardDef = CardDef { name: "Exiled Doomsayer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
