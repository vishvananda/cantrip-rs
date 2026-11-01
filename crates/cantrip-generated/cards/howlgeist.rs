// Howlgeist
// status: ready
// Creatures with power less than this creature's power can't block it.
// Undying (When this creature dies, if it had no +1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1 counter on it.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (*so_far).power.is_some_and(|v| (i64::from(v)) < (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !((q.chars(blocker).is(CardType::Creature) && q.chars(blocker).power.is_some_and(|v| (i64::from(v)) < (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) && Some(asker.source) == Some(attacker)) }

pub static CARD: CardDef = CardDef { name: "Howlgeist", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Wolf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::undying::trigger("Undying")], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_0_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
