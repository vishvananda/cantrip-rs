// Gaea's Revenge
// status: ready
// This spell can't be countered.
// Haste
// This creature can't be the target of nongreen spells or abilities from nongreen sources.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); subject == asker.source }
#[allow(unused_variables)] fn ability_2_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (by.spell && !((*by_source).colors.has(q.words(asker.source).color(Color::Green)))) }
#[allow(unused_variables)] fn ability_2_1_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); subject == asker.source }
#[allow(unused_variables)] fn ability_2_1_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(by.spell) && (!by.spell && !((*by_source).colors.has(q.words(asker.source).color(Color::Green))))) }

pub static CARD: CardDef = CardDef { name: "Gaea's Revenge", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(8), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeCountered,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_2_0_rule_by), applies: ability_2_0_rule_on },TargetingRule { kind: TargetingRuleKind::Forbid(ability_2_1_rule_by), applies: ability_2_1_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
