// Siege Behemoth
// status: planned
// requires: static rule CombatDamage AsUnblocked
// Hexproof
// As long as this creature is attacking, for each creature you control, you may have that creature assign its combat damage as though it weren't blocked.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().attackers.iter().any(|a| a.id == candidate))) && (((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.iter().any(|a| a.id == candidate) || q.was_attacking(candidate)))) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) }
planned_item!("static-rule-combat-damage-as-unblocked" => assigns_as_though_unblocked: ability_1_0_0_rule);

pub static CARD: CardDef = CardDef { name: "Siege Behemoth", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Hexproof]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
