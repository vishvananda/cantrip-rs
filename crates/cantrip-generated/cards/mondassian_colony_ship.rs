// Mondassian Colony Ship
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: operation TurnFaceDownAs
// requires: trigger event Player ChaosEnsues
// Whenever a creature attacks, it gets +1/+1 until end of turn for each other creature its controller controls that shares a creature type with it.
// Whenever chaos ensues, turn target creature face down. It becomes a 2/2 Cyberman artifact creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if q.chars(id).is(CardType::Creature) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((q.state().battlefield().iter().copied().filter(|&id| (!(item.subject.object() == Some(id)) && q.chars(id).is(CardType::Creature) && item.subject.object().map(|o: ObjectId| q.controller(o)) == Some(q.controller(id)) && item.subject.object().is_some_and(|o: ObjectId| { let (a, b) = (&q.chars(id), &q.chars(o)); a.subtypes.iter().any(|s| SubtypeSet::CREATURE_TYPES.has(s) && b.subtypes.has(s)) }))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16, ((q.state().battlefield().iter().copied().filter(|&id| (!(item.subject.object() == Some(id)) && q.chars(id).is(CardType::Creature) && item.subject.object().map(|o: ObjectId| q.controller(o)) == Some(q.controller(id)) && item.subject.object().is_some_and(|o: ObjectId| { let (a, b) = (&q.chars(id), &q.chars(o)); a.subtypes.iter().any(|s| SubtypeSet::CREATURE_TYPES.has(s) && b.subtypes.has(s)) }))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("operation-turn-face-down-as" => cantrip_mechanics::manifest_dread::turn_face_down(g, asker.source, &{ let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }));
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Mondassian Colony Ship", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[Subtype::Spacecraft]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever a creature attacks, it gets +1/+1 until end of turn for each other creature its controller controls that shares a creature type with it.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, turn target creature face down. It becomes a 2/2 Cyberman artifact creature.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
