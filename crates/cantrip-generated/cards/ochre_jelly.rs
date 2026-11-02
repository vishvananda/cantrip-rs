// Ochre Jelly
// status: planned
// requires: token copy entering with counters
// Trample
// This creature enters with X +1/+1 counters on it.
// Split — When this creature dies, if it had two or more +1/+1 counters on it, create a token that's a copy of it at the beginning of the next end step. The token enters with half that many +1/+1 counters on it, rounded down.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (i64::from(entering.x)).clamp(0, 32767) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) && subject.object().is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) >= (2i64)) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let created_3: Vec<ObjectId> = planned_value!("token copy entering with counters", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::Copy({ let q: &dyn Query = g; about(q, item, 0).first().copied() }, &[TokenCopyEffects::NoTokenCopyEffects]))], Arrival { tapped: false, counters: &[(CounterKind::PlusOne, { let q: &dyn Query = g; ((about(q, item, 0)).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::PlusOne))).sum::<i64>()).div_euclid(2) })], states: &[] })?);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) >= (2i64)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Ochre Jelly", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ooze]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Split — When this creature dies, if it had two or more +1/+1 counters on it, create a token that's a copy of it at the beginning of the next end step. The token enters with half that many +1/+1 counters on it, rounded down.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "Split — When this creature dies, if it had two or more +1/+1 counters on it, create a token that's a copy of it at the beginning of the next end step.")], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
