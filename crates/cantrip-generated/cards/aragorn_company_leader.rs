// Aragorn, Company Leader
// status: planned
// requires: counters the event put
// requires: instruction ChooseOne
// requires: object reference RingBearer
// requires: trigger event Player RingTempts
// Whenever the Ring tempts you, if you chose a creature other than Aragorn as your Ring-bearer, put your choice of a counter from among first strike, vigilance, deathtouch, and lifelink on Aragorn.
// Whenever you put one or more counters on Aragorn, put one of each of those kinds of counters on up to one other target creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { planned_value!("object reference RingBearer", None::<ObjectId> => q.ring_bearer(player)).is_some_and(|candidate: ObjectId| (!(Some(asker.source) == Some(candidate)) && q.chars(candidate).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_0, asker.source, vec![Label::Card(card_0, 0), Label::Card(card_0, 1), Label::Card(card_0, 2), Label::Card(card_0, 3)])?) {
0 => {
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::FirstStrike, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_2.extend(applied_1.clone());
}
1 => {
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::Vigilance, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_4.extend(applied_3.clone());
}
2 => {
let mut counters_5: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_5.push((object, CounterKind::Deathtouch, amount)); } }
let had_5: Vec<_> = counters_5.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_5); let applied_5: Vec<ObjectId> = counters_5.iter().zip(had_5).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_6.extend(applied_5.clone());
}
_ => {
let mut counters_7: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_7.push((object, CounterKind::Lifelink, amount)); } }
let had_7: Vec<_> = counters_7.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_7); let applied_7: Vec<ObjectId> = counters_7.iter().zip(had_7).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_8.extend(applied_7.clone());
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.event_subject().object() == Some(id)) && q.chars(id).is(CardType::Creature)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersPut { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if Some(asker.source) == Some(id) && Some(asker.controller) == Some(*by) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("counters-the-event-put" => let to_9: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let put_9: Vec<(CounterKind, i16)> = g.counters_put_by(item); let puts: Vec<(ObjectId, CounterKind, i16)> = to_9.iter().flat_map(|&o| put_9.iter().map(move |&(kind, amount)| (o, kind, 1i16))).collect(); g.add_counters_together(&puts););
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-ring-tempts" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::RingTempted { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Aragorn, Company Leader", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Ranger]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player RingTempts", EventMask(0) => EventMask(EventMask::RING_TEMPTED.0)), ability_0_event), ability_0, "Whenever the Ring tempts you, if you chose a creature other than Aragorn as your Ring-bearer, put your choice of a counter from among first strike, vigilance, deathtouch, and lifelink on Aragorn.").targets(&[]).condition(ability_0_if),triggered(when_asked(EventMask(EventMask::COUNTERS.0), ability_1_event), ability_1, "Whenever you put one or more counters on Aragorn, put one of each of those kinds of counters on up to one other target creature.").targets(ABILITY_1_TARGETS).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
