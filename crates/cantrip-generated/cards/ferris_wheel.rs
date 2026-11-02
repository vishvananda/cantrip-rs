// Ferris Wheel
// status: planned
// requires: ability Visit
// requires: duration UntilEvent
// requires: keyword action with a duration of its own
// requires: numeric expression Dice
// requires: trigger event Player RollsToVisit
// Visit — Choose target creature that hasn't been phased out with this Attraction. That creature phases out until you roll a 3 or less while rolling to visit your Attractions.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_visit_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && !(q.history().all().iter().any(|r| matches!(r.event, Event::PhasedOut { permanent, .. } if permanent == id && matches!(r.cause, Cause::Ability { source, .. } if Some(asker.source) == Some(source)))))))) }
static ABILITY_0_VISIT_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_visit_target_0)];
#[allow(unused_variables)] fn lasting_until_0(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0_visit(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("keyword-action-with-a-duration-of-its-own" => let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; for object in objects_0 { g.phase_out_until(object, planned_value!("duration UntilEvent", Duration::Indefinite => { let q: &dyn Query = g; Duration::UntilEvent({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) })); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-rolls-to-visit" => matcher: #[allow(unused_variables)] fn lasting_until_0(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::RolledToVisit { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) && (planned_value!("numeric expression Dice", 0i64 => q.last_roll(asker.controller).result)) <= (3i64) { return Some(Fired { subject, data }); } }
 None });
planned_item!("ability-visit" => abilities: cantrip_mechanics::attractions::visit_ability(spell(ABILITY_0_VISIT_TARGETS, ability_0_visit), None, ""));
planned_item!("duration-until-event" => lasting_until: LastingUntil { text: "you roll a 3 or less while rolling to visit your Attractions", ends: when_asked(planned_value!("trigger event Player RollsToVisit", EventMask(0) => EventMask(EventMask::ROLLED_TO_VISIT.0)), lasting_until_0) });

pub static CARD: CardDef = CardDef { name: "Ferris Wheel", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Attraction]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
