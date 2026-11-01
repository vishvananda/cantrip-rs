// Artifact Ward
// status: ready
// Enchant creature
// Enchanted creature can't be blocked by artifact creatures.
// Prevent all damage that would be dealt to enchanted creature by artifact sources.
// Enchanted creature can't be the target of abilities from artifact sources.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(attacker) && (q.chars(blocker).is(CardType::Artifact) && q.chars(blocker).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && q.chars(source).is(CardType::Artifact) && matches!(*target, Target::Object(o) if Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(o)) { return true; } }
 false }
#[allow(unused_variables)] fn ability_3_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); q.state().obj(asker.source).attached_to == Some(subject) }
#[allow(unused_variables)] fn ability_3_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(by.spell) && (!by.spell && (*by_source).is(CardType::Artifact))) }

pub static CARD: CardDef = CardDef { name: "Artifact Ward", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "Prevent all damage that would be dealt to enchanted creature by artifact sources.", applies: ability_2_applies, amount: None }], block_rules: &[ability_1_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_3_0_rule_by), applies: ability_3_0_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
