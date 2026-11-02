// Seraph
// status: planned
// requires: trigger event Player LosesControl
// Flying
// Whenever a creature dealt damage by this creature this turn dies, put that card onto the battlefield under your control at the beginning of the next end step. Sacrifice the creature when you lose control of this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && ({ let (outer_subject, outer_data) = (subject, data); q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Damage { source, target, amount, kind, controller, known, .. } = event { let (source, amount, combat, controller) = (*source, *amount, kind.is_combat(), *controller); let dealt = DamageDealt { source, target: *target, amount, combat, controller, known: *known }; let subject = *target; let data = [u32::from(amount), source.0]; if q.state().obj(source).zone.is(ZoneKind::Battlefield) && Some(asker.source) == Some(source) && matches!(*target, Target::Object(o) if Some(id) == Some(o)) { return Some(Fired { subject, data }); } }  None }).count() as i64 }) >= (1i64)) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn delayed_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
 Ok(()) }
fn delayed_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; ((about(q, item, 0)).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows_2: Vec<(ObjectId, PlayerId, bool)> = objects_2.iter().map(|&o| (o, player_2, false)).collect(); let came_2 = g.put_all_onto_battlefield_as(&rows_2)?; came_2.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 1), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
planned_item!("trigger-event-player-loses-control" => matcher: #[allow(unused_variables)] fn delayed_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ControlChanged { permanent, previous: player, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Seraph", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever a creature dealt damage by this creature this turn dies, put that card onto the battlefield under your control at the beginning of the next end step. Sacrifice the creature when you lose control of this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(planned_value!("trigger event Player LosesControl", EventMask(0) => EventMask(EventMask::CONTROL_CHANGED.0)), delayed_4_event), delayed_4, "Whenever a creature dealt damage by this creature this turn dies, put that card onto the battlefield under your control at the beginning of the next end step.").targets(&[]),delayed_trigger(delayed_1, "Whenever a creature dealt damage by this creature this turn dies, put that card onto the battlefield under your control at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
