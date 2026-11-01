// Canopy Cover
// status: ready
// Enchant creature
// Enchanted creature can't be blocked except by creatures with flying or reach.
// Enchanted creature can't be the target of spells or abilities your opponents control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(attacker) && !((q.chars(blocker).is(CardType::Creature) && (q.chars(blocker).keywords.has(Keyword::Flying) || q.chars(blocker).keywords.has(Keyword::Reach))))) }
#[allow(unused_variables)] fn ability_2_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); q.state().obj(asker.source).attached_to == Some(subject) }
#[allow(unused_variables)] fn ability_2_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&by.controller) }

pub static CARD: CardDef = CardDef { name: "Canopy Cover", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_1_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_2_0_rule_by), applies: ability_2_0_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
