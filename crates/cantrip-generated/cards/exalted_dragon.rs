// Exalted Dragon
// status: planned
// requires: static rule CombatCost other than mana
// Flying
// This creature can't attack unless you sacrifice a land. (This cost is paid as attackers are declared.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { attacker == asker.source && true }
planned_item!("static-rule-combat-cost-other-than-mana" => attack_cost_rules: AttackCost { applies: ability_1_0_rule, cost: "Cost" });

pub static CARD: CardDef = CardDef { name: "Exalted Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
