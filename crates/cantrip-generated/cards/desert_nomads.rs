// Desert Nomads
// status: ready
// Desertwalk
// Prevent all damage that would be dealt to this creature by Deserts.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_walk(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { attacker != source || !q.defending_player(attacker).is_some_and(|defending| q.state().battlefield().iter().any(|&land| q.controller(land) == defending && q.chars(land).is(CardType::Land) && q.chars(land).has_subtype(q.words(source).subtype(Subtype::Desert)))) }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Desert)) && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Desert Nomads", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Nomad]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all damage that would be dealt to this creature by Deserts", applies: ability_1_applies, amount: None }], block_rules: &[ability_0_walk], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
