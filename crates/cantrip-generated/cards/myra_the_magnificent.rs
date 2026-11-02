// Myra the Magnificent
// status: planned
// requires: duration WhileOnBattlefield
// requires: engine: a delayed ability on an event keeps no list
// requires: engine: no Attractions
// requires: event trigger carrying values
// requires: event trigger for WhileOnBattlefield
// requires: trigger event Player VisitsAttraction
// Whenever you cast an instant or sorcery spell from your hand, open an Attraction.
// {X}, {T}: Exile target instant or sorcery card with mana value X from your graveyard and choose an Attraction you control that doesn't have a midway counter on it. Put a midway counter on it. For as long as that Attraction is on the battlefield, whenever you visit it, copy the exiled card. You may cast the copy without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && cast_at(q, event, id).is_some_and(|cast| cast.from == ZoneKind::Hand && q.state().obj(cast.spell).owner == cast.player)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-attractions" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::attractions::open(g, player)?; });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && q.announced_x().map_or(true, |x| (i64::from(q.mana_value(id))) == (i64::from(x))) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn watch_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn watch_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_6 = { let q: &dyn Query = g; about(q, item, 1) }; let times_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_6: Vec<ObjectId> = Vec::new(); for _ in 0..times_6 { for &card in &originals_6 { copies_6.push(g.create_copy_in_exile(card, player_6)); } }
g.end_sentence();
let caster_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_7: Vec<ObjectId> = { let q: &dyn Query = g; (copies_6.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_7: u32 = 1; let mut played_7: Vec<ObjectId> = Vec::new();
while left_7 > 0 {
let mut open_7: Vec<ObjectId> = Vec::new(); for &c in &pool_7 { let ways = g.cast_ways_during_resolution_by(c, caster_7); if ways.iter().any(|w| true) { open_7.push(c); } }
let picked_7 = if open_7.is_empty() { None } else if open_7.len() > 1 || (true && (!true || pool_7.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_7, asker.source, Purpose::CastFree, pool_7.clone(), open_7.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_7, open_7[0], Purpose::CastFree)? { Some(open_7[0]) } else { None };
let Some(card_7) = picked_7 else { break; };
let ways_7: Vec<CastWay> = g.cast_ways_during_resolution_by(card_7, caster_7).into_iter().filter(|w| true).collect();
let done_7 = if ways_7.is_empty() { g.play_land_during_resolution(card_7)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_7, card_7, ways_7)?.map(|w| w.way); g.cast_during_resolution_as(card_7, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_7), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_7 { left_7 -= 1; played_7.push(g.state().obj(card_7).became.unwrap_or(card_7)); } else if true { break; } if done_7 || !(true && true) { pool_7.retain(|&c| c != card_7); }
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Attraction)) && Some(asker.controller) == Some(q.controller(id)) && (i64::from(q.state().obj(id).counters(CounterKind::Midway))) == (0i64))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::ChooseAsEnters, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
g.end_sentence();
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; ((selected_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::Midway, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
planned!("engine-a-delayed-ability-on-an-event-keeps-no-list" => if let Some(source) = Some(item.object) { let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; (selected_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE); about.extend({ let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  g.delayed_trigger_about(planned_value!("event trigger for WhileOnBattlefield", DelayedWhen::ThisTurn => DelayedWhen::Lasting(planned_value!("duration WhileOnBattlefield", Duration::Indefinite => Duration::WhileOnBattlefield(Result { result: ResultId(1), phase: After, index: 0 })))), source, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); });
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }
planned_item!("trigger-event-player-visits-attraction" => matcher: #[allow(unused_variables)] fn watch_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttractionVisited { permanent, player } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && planned_value!("event trigger carrying values", None::<ObjectId> => q.trigger_about(asker, 0).first().copied()) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Myra the Magnificent", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Performer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast an instant or sorcery spell from your hand, open an Attraction.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{X}, {T}: Exile target instant or sorcery card with mana value X from your graveyard and choose an Attraction you control that doesn't have a midway counter on it. Put a midway counter on it. For as long as that Attraction is on the battlefield, whenever you visit it, copy the exiled card. You may cast the copy without paying its mana cost.").targets(ABILITY_1_TARGETS).targets_after_x()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(planned_value!("trigger event Player VisitsAttraction", EventMask(0) => EventMask(EventMask::ATTRACTION_VISITED.0)), watch_5_event), watch_5, "For as long as that Attraction is on the battlefield, whenever you visit it, copy the exiled card.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
