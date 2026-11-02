// Warbringer
// status: planned
// requires: a keyword's cost changed
// Dash costs you pay cost {2} less (as long as this creature is on the battlefield).
// Dash {2}{R} (You may cast this spell for its dash cost. If you do, it gains haste, and it's returned from the battlefield to its owner's hand at the beginning of the next end step.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_0_0_keyword_cards_0(q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId) -> bool { player == asker.controller && true });
planned_item!("a-keyword-s-cost-changed" => #[allow(unused_variables)] fn ability_0_0_keyword_per_0(q: &dyn Query, asker: &Asker) -> i64 { 1 });
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Dash, applies: ability_0_0_keyword_cards_0, change: KeywordCostChange::Reduce(Cost::generic(2)), per: ability_0_0_keyword_per_0, first_each_turn: false });

pub static CARD: CardDef = CardDef { name: "Warbringer", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orc,Subtype::Berserker]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::dash::HASTE], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::dash::RETURN], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::dash::way(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Dash {2}{R}")], end_step_riders: cantrip_mechanics::dash::RIDERS, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
