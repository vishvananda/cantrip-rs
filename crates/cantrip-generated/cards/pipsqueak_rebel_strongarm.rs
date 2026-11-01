// Pipsqueak, Rebel Strongarm
// status: planned
// requires: static rule CombatGroup Attacker
// Pipsqueak can't attack alone unless he has a +1/+1 counter on him.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) > (0i64)))) && (source == affected) }
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) > (0i64)))) && id == asker.source }
#[allow(unused_variables)] fn ability_0_0_0_rule_counted(q: &dyn Query, asker: &Asker, id: ObjectId, other: ObjectId) -> bool { true }
planned_item!("static-rule-combat-group-attacker" => combat_groups: CombatGroupRule { role: CombatRole::Attacker, applies: ability_0_0_0_rule, counted: ability_0_0_0_rule_counted, others_only: false, count: 2..=u32::MAX });

pub static CARD: CardDef = CardDef { name: "Pipsqueak, Rebel Strongarm", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Rebel,Subtype::Ally]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
