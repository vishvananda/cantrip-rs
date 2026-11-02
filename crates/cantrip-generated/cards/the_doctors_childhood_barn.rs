// The Doctor's Childhood Barn
// status: planned
// requires: card type Plane
// requires: engine: no effect phases a permanent in
// requires: layout Planar
// requires: operation GameEffect
// requires: reflexive ability about a list of objects
// requires: subtype Gallifrey
// requires: trigger event Player ChaosEnsues
// requires: trigger event Player Planeswalks
// Creatures enter tapped.
// Whenever chaos ensues, for each opponent, choose up to one target nonland permanent that opponent controls. Untap those permanents. They phase out. They can't phase in for as long as The Doctor's Childhood Barn remains face up. When a player planeswalks, those permanents phase in.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && q.chars(id).is(CardType::Creature) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_0_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.tapped = true; } } }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::controlled_by_slot_opponent(q, id)) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (!(q.chars(id).is(CardType::Land))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, TargetSpec::EACH_OPPONENT)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
#[allow(unused_variables)] fn lasting_0(q: &dyn Query, e: &ContinuousEffect) -> bool { let asker = &Asker::new(e.source, e.controller); is_live(q, e.source) && q.is_on_battlefield(e.source) && !(q.state().obj(e.source).face_down) }
#[allow(unused_variables)] fn delayed_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for bound_4294967290 in { let q: &dyn Query = g; planned_value!("reflexive ability about a list of objects", Vec::<ObjectId>::new() => about(q, item, acted_1.clone())) } {
planned!("engine-no-effect-phases-a-permanent-in" => if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967290).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.phase_in(&[object]); });
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_0 { g.untap(object); } });
g.end_sentence();
let acted_1: Vec<ObjectId> = { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() };
let objects_1 = { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; g.phase_out(&objects_1);
g.end_sentence();
planned!("operation-game-effect" => cantrip_mechanics::game_rules::for_a_time(g, asker, { let q: &dyn Query = g; Duration::AsLongAs({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) }, GameEffect::PermanentsCantPhaseIn({ let q: &dyn Query = g; acted_1.clone() })));
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-planeswalks" => matcher: #[allow(unused_variables)] fn delayed_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Planeswalked { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "The Doctor's Childhood Barn", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Gallifrey", Subtype::Plains => Subtype::Gallifrey)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, for each opponent, choose up to one target nonland permanent that opponent controls. Untap those permanents. They phase out. They can't phase in for as long as The Doctor's Childhood Barn remains face up. When a player planeswalks, those permanents phase in.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_0_applies, ability_0_apply, "Creatures enter tapped.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(planned_value!("trigger event Player Planeswalks", EventMask(0) => EventMask(EventMask::PLANESWALKED.0)), delayed_4_event), delayed_4, "When a player planeswalks, those permanents phase in.").targets(&[])], enters: None, modal: None, enchant: None, lasting: &[Lasting { text: "its condition holds", holds: lasting_0 }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
