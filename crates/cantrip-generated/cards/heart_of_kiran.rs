// Heart of Kiran
// status: planned
// requires: a keyword's cost changed
// Flying, vigilance
// Crew 3 (Tap any number of creatures you control with total power 3 or more: This Vehicle becomes an artifact creature until end of turn.)
// You may remove a loyalty counter from a planeswalker you control rather than pay Heart of Kiran's crew cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_cost_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(id))) }
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_3_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player == asker.controller && Some(asker.source) == Some(card) });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_3_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Crew, applies: ability_3_0_keyword_cards_0, change: KeywordCostChange::Alternate(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(remove_kind_counters_among(CounterKind::Loyalty, 1, ability_3_0_cost_0))), per: ability_3_0_keyword_per_0, first_each_turn: false });

pub static CARD: CardDef = CardDef { name: "Heart of Kiran", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::crew::crew(3, "Crew 3")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
