// Karmic Justice
// status: planned
// requires: event value in a target filter beyond its subject
// requires: trigger event Caused
// Whenever a spell or ability an opponent controls destroys a noncreature permanent you control, you may destroy target permanent that opponent controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((true && Some(PlayerId(planned_value!("event value in a target filter beyond its subject", [u32::MAX; 2] => q.event_data())[1] as u8)) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let objects_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
hoisted_2.extend(destroyed_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-caused" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, reason: ZoneReason::Destroy | ZoneReason::DestroyNoRegen, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; let data = [data[0], match q.event_cause() { Cause::Spell(c) | Cause::Ability { source: c, .. } => u32::from(q.controller(c).0), _ => u32::MAX }]; if from.is(ZoneKind::Battlefield) && (!(q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(q.controller(id))) && match q.event_cause() { Cause::Spell(causer) | Cause::Ability { source: causer, .. } => (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(causer)), _ => false } { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Karmic Justice", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Caused", EventMask(0) => EventMask(EventMask::ZONE_CHANGE.0)), ability_0_event), ability_0, "Whenever a spell or ability an opponent controls destroys a noncreature permanent you control, you may destroy target permanent that opponent controls.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
