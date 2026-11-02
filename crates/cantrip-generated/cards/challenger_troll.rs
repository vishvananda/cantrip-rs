// Challenger Troll
// status: ready
// Each creature you control with power 4 or greater can't be blocked by more than one creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)) && (*so_far).power.is_some_and(|v| (i64::from(v)) >= (4i64))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, id: ObjectId) -> Option<usize> { let asker = &Asker::new(source, q.controller(source)); ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && q.chars(id).power.is_some_and(|v| (i64::from(v)) >= (4i64)))).then_some(1) }

pub static CARD: CardDef = CardDef { name: "Challenger Troll", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Troll]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, blocker_limits: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
