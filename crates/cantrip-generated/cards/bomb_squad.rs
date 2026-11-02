// Bomb Squad
// status: planned
// requires: trigger event ObjectBecomes
// {T}: Put a fuse counter on target creature.
// At the beginning of your upkeep, put a fuse counter on each creature with a fuse counter on it.
// Whenever a creature has four or more fuse counters on it, remove all fuse counters from it and destroy it. That creature deals 4 damage to its controller.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Fuse, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (i64::from(q.state().obj(id).counters(CounterKind::Fuse))) > (0i64))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::Fuse, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = i16::MAX; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, CounterKind::Fuse, amount_2_0)); } });
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
let source_4 = { let q: &dyn Query = g; item.subject.object() }; let amount_4 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; (item.subject.object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-object-becomes" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StateChecked { object } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if (q.chars(id).is(CardType::Creature) && (i64::from(q.state().obj(id).counters(CounterKind::Fuse))) >= (4i64)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Bomb Squad", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dwarf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: Put a fuse counter on target creature.").targets(ABILITY_0_TARGETS),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, put a fuse counter on each creature with a fuse counter on it.").targets(&[]),triggered(when_asked(planned_value!("trigger event ObjectBecomes", EventMask(0) => EventMask(EventMask::OBJECT_STATE.0)), ability_2_event), ability_2, "Whenever a creature has four or more fuse counters on it, remove all fuse counters from it and destroy it. That creature deals 4 damage to its controller.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
