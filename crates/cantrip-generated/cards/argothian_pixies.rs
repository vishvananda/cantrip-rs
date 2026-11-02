// Argothian Pixies
// status: ready
// This creature can't be blocked by artifact creatures.
// Prevent all damage that would be dealt to this creature by artifact creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(attacker == asker.source && (q.chars(blocker).is(CardType::Artifact) && q.chars(blocker).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Artifact) && q.chars(source).is(CardType::Creature)) && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Argothian Pixies", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all damage that would be dealt to this creature by artifact creatures", applies: ability_1_applies, amount: None }], block_rules: &[ability_0_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
