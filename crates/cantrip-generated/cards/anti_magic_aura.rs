// Anti-Magic Aura
// status: planned
// requires: static rule Prohibit BeAttachedBy
// Enchant creature
// Enchanted creature can't be the target of spells and can't be enchanted by other Auras.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); q.state().obj(asker.source).attached_to == Some(subject) }
#[allow(unused_variables)] fn ability_1_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (by.spell && true) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; q.state().obj(asker.source).attached_to == Some(id) }
planned_item!("static-rule-prohibit-be-attached-by" => statics: object_rule(ability_1_1_rule, Rule::Cant(Action::BeAttachedBy { attachments: |q: &dyn Query, asker: &Asker, attachment: ObjectId| (!(Some(asker.source) == Some(attachment)) && q.chars(attachment).has_subtype(q.words(asker.source).subtype(Subtype::Aura))), keep_existing: false })));

pub static CARD: CardDef = CardDef { name: "Anti-Magic Aura", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_1_0_rule_by), applies: ability_1_0_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
