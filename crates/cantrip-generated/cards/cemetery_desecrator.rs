// Cemetery Desecrator
// status: planned
// requires: counters of a kind chosen as the effect resolves
// requires: modal announcement
// Menace
// When this creature enters or dies, exile another card from a graveyard. When you do, choose one —
// • Remove X counters from target permanent, where X is the mana value of the exiled card.
// • Target creature an opponent controls gets -X/-X until end of turn, where X is the mana value of the exiled card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_3_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static REFLEXIVE_3_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_3_mode_0_target_0)];
fn reflexive_3_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut removed_4: i64 = 0;
let objects_4_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
planned!("counters-of-a-kind-chosen-as-the-effect-resolves" => removed_4 += i64::from(cantrip_mechanics::effects::remove_chosen_counters(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, &objects_4_0, None, RemovedCounters::Each(cantrip_mechanics::select::Count::AsManyAsCan((item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).max(0) as u32)))?););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn reflexive_3_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static REFLEXIVE_3_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_3_mode_1_target_0)];
fn reflexive_3_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_5 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::EndOfTurn;
let kind_5_0_0 = { let q: &dyn Query = g; EffectKind::Pump((((-1i64 * item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))))).clamp(-32768, 32767) as i16).min(0), (((-1i64 * item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))))).clamp(-32768, 32767) as i16).min(0)) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
g.end_sentence();
 Ok(()) }
static REFLEXIVE_3_MODES: ModalDef = modes(&[mode(reflexive_3_mode_0, "Remove X counters from target permanent, where X is the mana value of the exiled card.").targets(REFLEXIVE_3_MODE_0_TARGETS),mode(reflexive_3_mode_1, "Target creature an opponent controls gets -X/-X until end of turn, where X is the mana value of the exiled card.").targets(REFLEXIVE_3_MODE_1_TARGETS)]).choose(1, 1);
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(Some(asker.source) == Some(id))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Exile, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let moved_1 = g.exile_all(&objects_1);
if !moved_1.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Cemetery Desecrator", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters or dies, exile another card from a graveyard. When you do, choose one — Remove X counters from target permanent, where X is the mana value of the exiled card; or Target creature an opponent controls gets -X/-X until end of turn, where X is the mana value of the exiled card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[planned_value!("modal announcement", delayed_trigger(reflexive_3, "When this creature enters or dies, exile another card from a graveyard. When you do, choose one — Remove X counters from target permanent, where X is the mana value of the exiled card; or Target creature an opponent controls gets -X/-X until end of turn, where X is the mana value of the exiled card.").targets(&[]) => delayed_trigger(reflexive_3, "When this creature enters or dies, exile another card from a graveyard. When you do, choose one — Remove X counters from target permanent, where X is the mana value of the exiled card; or Target creature an opponent controls gets -X/-X until end of turn, where X is the mana value of the exiled card.").targets(&[]).modes(&REFLEXIVE_3_MODES))], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
