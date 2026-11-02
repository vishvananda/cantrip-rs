// Fraying Line
// status: ready
// When this artifact enters, put a rope counter on target creature you control.
// At the beginning of each player's upkeep, that player may pay {2}. If they do, they put a rope counter on a creature they control. Otherwise, exile this artifact and each creature without a rope counter on it, then remove all rope counters from all creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Rope, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_1 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
{ let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let chosen = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::PutCounters(CounterKind::Rope, amount.clamp(0, 255) as u8), candidates, cantrip_mechanics::select::Selection::of(count, &[]))?; for object in chosen { counters_2.push((object, CounterKind::Rope, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_3.extend(applied_2.clone());
} else {
let objects_4 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in Some(asker.source).into_iter().collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (i64::from(q.state().obj(id).counters(CounterKind::Rope))) == (0i64))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
let mut removed_5: i64 = 0;
let objects_5_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() };
let amount_5_0 = i16::MAX; cantrip_api::instruction(g, |g| { for object in objects_5_0 { removed_5 += i64::from(g.remove_counters(object, CounterKind::Rope, amount_5_0)); } });
g.end_sentence();
hoisted_6.extend(moved_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Fraying Line", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this artifact enters, put a rope counter on target creature you control.").targets(ABILITY_0_TARGETS),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each player's upkeep, that player may pay {2}. If they do, they put a rope counter on a creature they control. Otherwise, exile this artifact and each creature without a rope counter on it, then remove all rope counters from all creatures.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
