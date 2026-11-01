// Hollow Warrior
// status: planned
// requires: static rule CombatCost Block
// requires: static rule CombatCost other than mana
// This creature can't attack or block unless you tap an untapped creature you control not declared as an attacking or blocking creature this combat. (This cost is paid as attackers or blockers are declared.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { attacker == asker.source && true }
#[allow(unused_variables)] fn ability_0_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
planned_item!("static-rule-combat-cost-other-than-mana" => attack_cost_rules: AttackCost { applies: ability_0_0_rule, cost: "Cost" });
planned_item!("static-rule-combat-cost-block" => block_costs: ability_0_1_rule);

pub static CARD: CardDef = CardDef { name: "Hollow Warrior", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Golem,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
