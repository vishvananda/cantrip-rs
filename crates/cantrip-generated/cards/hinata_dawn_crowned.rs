// Hinata, Dawn-Crowned
// status: planned
// requires: spell cost rule with terms
// Flying, trample
// Spells you cast cost {1} less to cast for each target.
// Spells your opponents cast cost {1} more to cast for each target.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_3_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller }
planned_item!("spell-cost-rule-with-terms" => statics: player_rule(ability_2_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true, change: CostModifier::Scaled { change: CostChange::Reduce(&[CostReductionSymbol::CostReduceGeneric(1)]), per: CostScale::EachTarget } }));
planned_item!("spell-cost-rule-with-terms" => statics: player_rule(ability_3_0_applies, PlayerRule::SpellCost { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true, change: CostModifier::Scaled { change: CostChange::Increase(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), per: CostScale::EachTarget } }));

pub static CARD: CardDef = CardDef { name: "Hinata, Dawn-Crowned", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kirin,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
