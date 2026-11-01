// Mindbender Spores
// status: planned
// requires: counter kind Fungus
// requires: granted Conditional
// Defender (This creature can't attack.)
// Flying
// Whenever this creature blocks a creature, put four fungus counters on that creature. The creature gains "This creature doesn't untap during your untap step if it has a fungus counter on it" and "At the beginning of your upkeep, remove a fungus counter from this creature."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, .. } = event { let id = *blocker; let other = *attacker; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if Some(asker.source) == Some(id) && q.chars(other).is(CardType::Creature) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn granted_2_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(planned_value!("counter kind Fungus", CounterKind::Charge => CounterKind::Fungus)))) > (0i64))) && (source == affected) }
#[allow(unused_variables)] fn granted_2_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(planned_value!("counter kind Fungus", CounterKind::Charge => CounterKind::Fungus)))) > (0i64))) && id == asker.source }
#[allow(unused_variables)] fn granted_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn granted_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_4: i64 = 0;
let objects_4_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_4_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_4_0 { removed_4 += i64::from(g.remove_counters(object, planned_value!("counter kind Fungus", CounterKind::Charge => CounterKind::Fungus), amount_4_0)); } });
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (4i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, planned_value!("counter kind Fungus", CounterKind::Charge => CounterKind::Fungus), amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kinds_1 = { let mut all = Vec::new(); for &object in &objects_1 { let kind = { let q: &dyn Query = g; EffectKind::AddAbility(planned_value!("granted Conditional", AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) => AbilityRef::granted_static({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; all.push((object, kind)); let kind = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) }; all.push((object, kind));  } all }; if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_1 { g.add_effect(source, object, kind, duration_1); } }); }
g.end_sentence();
 Ok(()) }
planned_item!("granted-conditional" => granted_rules: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[granted_2_0_0_rule], enters: None, ..CardDef::BLANK });

pub static CARD: CardDef = CardDef { name: "Mindbender Spores", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fungus,Subtype::Wall]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_2_event), ability_2, "Whenever this creature blocks a creature, put four fungus counters on that creature. The creature gains \"This creature doesn't untap during your untap step if it has a fungus counter on it\" and \"At the beginning of your upkeep, remove a fungus counter from this creature.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_3_event), granted_3, "At the beginning of your upkeep, remove a fungus counter from this creature.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
