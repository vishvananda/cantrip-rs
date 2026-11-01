// Loyal Pegasus
// status: planned
// requires: static rule CombatGroup Attacker
// requires: static rule CombatGroup Blocker
// Flying
// This creature can't attack or block alone.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_1_0_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { true }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_1_1_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { true }
planned_item!("static-rule-combat-group-attacker" => combat_groups: CombatGroupRule { role: CombatRole::Attacker, applies: ability_1_0_rule, counted: ability_1_0_rule_counted, others_only: false, count: 2..=u32::MAX });
planned_item!("static-rule-combat-group-blocker" => combat_groups: CombatGroupRule { role: CombatRole::Blocker, applies: ability_1_1_rule, counted: ability_1_1_rule_counted, others_only: false, count: 2..=u32::MAX });

pub static CARD: CardDef = CardDef { name: "Loyal Pegasus", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Pegasus]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
