// Temporal Distortion
// status: planned
// requires: counter kind Hourglass
// Whenever a creature or land becomes tapped, put an hourglass counter on it.
// Each permanent with an hourglass counter on it doesn't untap during its controller's untap step.
// At the beginning of each player's upkeep, remove all hourglass counters from permanents that player controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Tapped { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if (q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Land)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, planned_value!("counter kind Hourglass", CounterKind::Charge => CounterKind::Hourglass), amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (i64::from(q.state().obj(affected).counters(planned_value!("counter kind Hourglass", CounterKind::Charge => CounterKind::Hourglass)))) > (0i64) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (i64::from(q.state().obj(id).counters(planned_value!("counter kind Hourglass", CounterKind::Charge => CounterKind::Hourglass)))) > (0i64) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (true && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>() };
let amount_1_0 = i16::MAX; cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += i64::from(g.remove_counters(object, planned_value!("counter kind Hourglass", CounterKind::Charge => CounterKind::Hourglass), amount_1_0)); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Temporal Distortion", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), ability_0_event), ability_0, "Whenever a creature or land becomes tapped, put an hourglass counter on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of each player's upkeep, remove all hourglass counters from permanents that player controls.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_1_0_rule], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
