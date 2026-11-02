// Mogg Jailer
// status: ready
// This creature can't attack if defending player controls an untapped creature with power 2 or less.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
#[allow(unused_variables)] fn ability_0_0_rule_attack(q: &dyn Query, asker: &Asker, attacker: ObjectId, at: Target) -> bool { if !ability_0_0_rule(q, asker, attacker) { return true; } let Some(defender) = defending_player_of(q.state(), at) else { return true; }; !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(defender).unwrap_or(PlayerId(u8::MAX)) && (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) <= (2i64)))).count() >= 1) }

pub static CARD: CardDef = CardDef { name: "Mogg Jailer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_0_0_rule_attack], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
