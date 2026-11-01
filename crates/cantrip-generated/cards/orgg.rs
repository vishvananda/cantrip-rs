// Orgg
// status: planned
// requires: static rule CantAttackDefenderIf
// Trample
// This creature can't attack if defending player controls an untapped creature with power 3 or greater.
// This creature can't block creatures with power 3 or greater.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(blocker == asker.source && (q.chars(attacker).is(CardType::Creature) && q.chars(attacker).power.is_some_and(|v| (i64::from(v)) >= (3i64)))) }
planned_item!("static-rule-cant-attack-defender-if" => statics: object_rule(ability_1_0_rule, Rule::CantAttackDefenderIf(|q: &dyn Query, asker: &Asker, player: PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)).unwrap_or(PlayerId(u8::MAX)) && (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) >= (3i64)))).count() >= 1)));

pub static CARD: CardDef = CardDef { name: "Orgg", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orgg]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_2_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
