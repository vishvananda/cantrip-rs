// Battle of Hoover Dam
// status: ready
// As this enchantment enters, choose NCR or Legion.
// • NCR — At the beginning of your end step, return target creature card with mana value 3 or less from your graveyard to the battlefield with a finality counter on it.
// • Legion — Whenever a creature you control dies, put two +1/+1 counters on target creature you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; let chosen_0: u8 = entering.chosen_option.map_or(0, |chosen| chosen.index);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (3i64)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_1_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_0_target_0)];
#[allow(unused_variables)] fn ability_1_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) && cantrip_mechanics::choose::chosen_option(q, asker, &CARD) == Some(0) { return Some(Fired { subject, data }); } }
 None }
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let counters = [(CounterKind::Finality, { let q: &dyn Query = g; (1i64).max(0) as i16 })]; g.put_all_onto_battlefield_with_counters(&rows_1, &counters)? };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_0_target_0)];
#[allow(unused_variables)] fn ability_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && cantrip_mechanics::choose::chosen_option(q, asker, &CARD) == Some(1) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Battle of Hoover Dam", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_0_event), ability_1_0, "As this enchantment enters, choose NCR or Legion. NCR — At the beginning of your end step, return target creature card with mana value 3 or less from your graveyard to the battlefield with a finality counter on it; or Legion — Whenever a creature you control dies, put two +1/+1 counters on target creature you control.").targets(ABILITY_1_0_TARGETS),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_0_event), ability_2_0, "As this enchantment enters, choose NCR or Legion. NCR — At the beginning of your end step, return target creature card with mana value 3 or less from your graveyard to the battlefield with a finality counter on it; or Legion — Whenever a creature you control dies, put two +1/+1 counters on target creature you control.").targets(ABILITY_2_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["NCR", "Legion"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
