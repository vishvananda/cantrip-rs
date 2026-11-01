// Gorm the Great
// status: planned
// requires: static rule RequireCombat BeBlocked
// Partner with Virtus the Veiled (When this creature enters, target player may put Virtus into their hand from their library, then shuffle.)
// Vigilance
// Gorm must be blocked if able, and Gorm must be blocked by two or more creatures if able.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); attacker == asker.source }
#[allow(unused_variables)] fn ability_2_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
planned_item!("static-rule-require-combat-be-blocked" => statics: object_rule(ability_2_1_rule, Rule::Must(Action::BeBlocked(|q: &dyn Query, asker: &Asker, blocker: ObjectId| true), Required::AtLeast(2i64))));

pub static CARD: CardDef = CardDef { name: "Gorm the Great", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, must_be_blocked: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
