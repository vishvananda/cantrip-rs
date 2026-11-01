// Bartel Runeaxe
// status: ready
// Vigilance
// Bartel Runeaxe can't be the target of Aura spells.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); subject == asker.source }
#[allow(unused_variables)] fn ability_1_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (by.spell && (*by_source).has_subtype(q.words(asker.source).subtype(Subtype::Aura))) }

pub static CARD: CardDef = CardDef { name: "Bartel Runeaxe", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_1_0_rule_by), applies: ability_1_0_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
