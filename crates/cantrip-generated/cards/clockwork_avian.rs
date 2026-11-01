// Clockwork Avian
// status: planned
// requires: power/toughness counter other than ±1/±1
// Flying
// This creature enters with four +1/+0 counters on it.
// At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.
// {X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than four. Activate only during your upkeep.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (4i64).max(0) as i16 }; entering.counters[planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus1Plus0) as usize] = entering.counters[planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus1Plus0) as usize].saturating_add(more);
 Ok(()) }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| ((cantrip_mechanics::history::this_combat(q).iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(candidate) == Some(id) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) || (cantrip_mechanics::history::this_combat(q).iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(candidate) == Some(id) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64))) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::EndOfCombat) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_1_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += i64::from(g.remove_counters(object, planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus1Plus0), amount_1_0)); } });
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let object_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; (i64::from(item.x)).clamp(0, 32767) as i16 }; let cap_2 = { let q: &dyn Query = g; (4i64).max(0) as i16 }; if let Some(object) = object_2 { let room = cap_2.saturating_sub(g.state().obj(object).counters(planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus1Plus0))).max(0); g.add_counters(object, planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus1Plus0), amount_2.min(room)); }
g.end_sentence();
 Ok(()) }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller).is_some_and(|player: PlayerId| q.state().turn.active == player && q.state().turn.step() == Step::Upkeep) }

pub static CARD: CardDef = CardDef { name: "Clockwork Avian", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At end of combat, if this creature attacked or blocked this combat, remove a +1/+0 counter from it.").targets(&[]).condition(ability_2_if),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_3, "{X}, {T}: Put up to X +1/+0 counters on this creature. This ability can't cause the total number of +1/+0 counters on this creature to be greater than four. Activate only during your upkeep.").targets(&[]).condition(ability_3_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
