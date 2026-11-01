// Okk
// status: planned
// requires: static rule CombatGroup Attacker
// requires: static rule CombatGroup Blocker
// This creature can't attack unless a creature with greater power also attacks.
// This creature can't block unless a creature with greater power also blocks.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_0_0_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { (q.chars(other).is(CardType::Creature) && q.chars(other).power.is_some_and(|v| (i64::from(v)) > (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_1_0_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { (q.chars(other).is(CardType::Creature) && q.chars(other).power.is_some_and(|v| (i64::from(v)) > (Some(id).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
planned_item!("static-rule-combat-group-attacker" => combat_groups: CombatGroupRule { role: CombatRole::Attacker, applies: ability_0_0_rule, counted: ability_0_0_rule_counted, others_only: true, count: 1..=u32::MAX });
planned_item!("static-rule-combat-group-blocker" => combat_groups: CombatGroupRule { role: CombatRole::Blocker, applies: ability_1_0_rule, counted: ability_1_0_rule_counted, others_only: true, count: 1..=u32::MAX });

pub static CARD: CardDef = CardDef { name: "Okk", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
