// Ugin's Conjurant
// status: planned
// requires: engine: no static prevention with an effect besides
// This creature enters with X +1/+1 counters on it.
// If damage would be dealt to this creature while it has a +1/+1 counter on it, prevent that damage and remove that many +1/+1 counters from this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (i64::from(entering.x)).min(i16::MAX.into()) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; subject.object().into_iter().collect::<Vec<ObjectId>>() };
let amount_1_0 = { let q: &dyn Query = g; (i64::from(data[0])).min(i16::MAX.into()) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += i64::from(g.remove_counters(object, CounterKind::PlusOne, amount_1_0)); } });
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_1_0_then(g: &mut dyn Game, asker: &Asker, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_1_0_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) && Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) > (0i64)) { return true; } }
 false }
planned_item!("engine-no-static-prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If damage would be dealt to this creature while it has a +1/+1 counter on it, prevent that damage and remove that many +1/+1 counters from this creature.", applies: ability_1_0_applies, amount: None, then: Some(ability_1_0_then) });

pub static CARD: CardDef = CardDef { name: "Ugin's Conjurant", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Monk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
