// Cosmos Charger
// status: planned
// requires: a keyword's cost changed
// requires: player rule KeywordAccess
// Flash
// Flying
// Foretelling cards from your hand costs {1} less and can be done on any player's turn.
// Foretell {2}{U} (During your turn, you may pay {2} and exile this card from your hand face down. Cast it on a later turn for its foretell cost.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("a-keyword-s-cost-changed" => keyword_costs: KeywordCostRule { keyword: CostedKeyword::Foretell, applies: |q, asker, player, card| player == asker.controller && true, change: KeywordCostChange::Reduce(Cost::generic(1)), per: |q, asker| 1, first_each_turn: false });
planned_item!("player-rule-keyword-access" => statics: player_rule(ability_2_1_applies, PlayerRule::KeywordAccess { keyword: CostedKeyword::Foretell, cards: |q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| true, from: ZoneKind::Hand, during: Some(|q: &dyn Query, asker: &Asker, player: PlayerId| true) }));

pub static CARD: CardDef = CardDef { name: "Cosmos Charger", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Horse,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::foretell::ABILITY], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, foretell: Some(ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
