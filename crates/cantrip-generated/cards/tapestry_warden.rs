// Tapestry Warden
// status: planned
// requires: static rule TapCostUsesToughness
// Vigilance
// Each creature you control with toughness greater than its power assigns combat damage equal to its toughness rather than its power.
// Each creature you control with toughness greater than its power stations permanents using its toughness rather than its power.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (*so_far).toughness.is_some_and(|v| (i64::from(v)) > (Some(affected).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Creature) && q.chars(id).toughness.is_some_and(|v| (i64::from(v)) > (Some(id).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (*so_far).toughness.is_some_and(|v| (i64::from(v)) > (Some(affected).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; (q.chars(id).is(CardType::Creature) && q.chars(id).toughness.is_some_and(|v| (i64::from(v)) > (Some(id).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))))) }
planned_item!("static-rule-tap-cost-uses-toughness" => statics: object_rule(ability_2_0_rule, Rule::TapCostUsesToughness(TapCost::Station)));

pub static CARD: CardDef = CardDef { name: "Tapestry Warden", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Robot,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, damage_by_toughness: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
