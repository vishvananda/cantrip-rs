// Grakmaw, Skyclave Ravager
// status: ready
// Grakmaw enters with three +1/+1 counters on it.
// Whenever another creature you control dies, if it had a +1/+1 counter on it, put a +1/+1 counter on Grakmaw.
// When Grakmaw dies, create an X/X black and green Hydra creature token, where X is the number of +1/+1 counters on Grakmaw.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (3i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))) && subject.object().is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) > (0i64)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::PlusOne))) > (0i64)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::PlusOne, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Hydra).name, token_words: token!(Hydra).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hydra]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(20)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let power_2 = { let q: &dyn Query = g; ((item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::PlusOne))).sum::<i64>()).clamp(0, 32767) as i16 }; let toughness_2 = { let q: &dyn Query = g; ((item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::PlusOne))).sum::<i64>()).clamp(0, 32767) as i16 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut values_2 = Copiable::of(g.card_id(&token_3::CARD)).sized(power_2, toughness_2); values_2.words = g.words(asker.source); let mut created_2: Vec<ObjectId> = Vec::new(); for player in players_2 { created_2.extend(g.create_token_copies_tapped(values_2, player, count_2, false)?); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Grakmaw, Skyclave Ravager", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hydra,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever another creature you control dies, if it had a +1/+1 counter on it, put a +1/+1 counter on Grakmaw.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Grakmaw dies, create an X/X black and green Hydra creature token, where X is the number of +1/+1 counters on Grakmaw.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD];
