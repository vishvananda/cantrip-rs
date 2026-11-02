// Kharasha Foothills
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: per-player instruction with several steps
// requires: subtype Mongseng
// requires: trigger event Player ChaosEnsues
// Whenever a creature you control attacks a player, for each other opponent, you may create a token that's a copy of that creature, tapped and attacking that opponent. Exile those tokens at the beginning of the next end step.
// Whenever chaos ensues, you may sacrifice any number of creatures. If you do, Kharasha Foothills deals that much damage to target creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) && match *defender { Target::Player(p) => true, _ => false } { return Some(Fired { subject, data }); } }
 None }
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_0, _) in players_0.iter().copied().enumerate() { let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let original_2 = { let q: &dyn Query = g; item.subject.object() }; let values_2 = { let q: &dyn Query = g; original_2.map(|original: ObjectId| q.copiable_values(original)) }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_2 = match values_2 { Some(values) => g.create_token_copies_tapped(values, player_2, count_2, true)?, None => Vec::new() };
let defender_2 = Target::Player({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }); g.enter_attacking(&created_2, defender_2);
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_2.clone() }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
hoisted_6.extend(created_2.clone());
}
 each_7[index_0] = hoisted_6.clone(); all_7.extend(each_7[index_0].iter().copied());
   } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let payer_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_8, after_8, before_8, picked_8): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let objects_9 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_9: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_9 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; for player in players_9 { let eligible: Vec<ObjectId> = objects_9.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_9.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() < n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_9.clone(), &[]))? }; for object in objects_9.iter().copied().filter(|o| taken.contains(o)).collect::<Vec<ObjectId>>() { picks_9.push((player, object)); } }
let sacrificed_9 = g.sacrifice_each(&picks_9);
let paid_9 = !picks_9.is_empty();
(paid_9, { let q: &dyn Query = g; (sacrificed_9).clone() }, { let q: &dyn Query = g; (objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_8 as i64) != 0) } {
let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let amount_10 = { let q: &dyn Query = g; ((before_8.clone()).len() as i64).clamp(0, 65535) as u16 }; let recipients_10 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_10 { let batch: Vec<(ObjectId, Target, u16)> = recipients_10.iter().map(|t| (source, *t, amount_10)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Kharasha Foothills", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Mongseng", Subtype::Plains => Subtype::Mongseng)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever a creature you control attacks a player, for each other opponent, you may create a token that's a copy of that creature, tapped and attacking that opponent. Exile those tokens at the beginning of the next end step.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, you may sacrifice any number of creatures. If you do, Kharasha Foothills deals that much damage to target creature.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, "Exile those tokens at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
