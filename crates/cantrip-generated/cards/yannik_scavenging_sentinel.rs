// Yannik, Scavenging Sentinel
// status: planned
// requires: target with a divided amount read as chosen
// Partner with Nikara, Lair Scavenger
// Vigilance
// When Yannik enters, exile another creature you control until Yannik leaves the battlefield. When you do, distribute X +1/+1 counters among any number of target creatures, where X is the exiled creature's power.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static REFLEXIVE_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_3_target_0).between(1, TargetSpec::COUNTED)];
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::counters_divided(g, item, CounterKind::PlusOne);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Exile, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let by_1 = g.linked(item); let moved_1 = g.exile_with(&objects_1, by_1, true);
if !moved_1.clone().is_empty() && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).get(0 as usize).copied() }.map_or(Target::None, Target::Object), (({ let q: &dyn Query = g; (objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).get(0 as usize).copied().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))) } as i64)).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Yannik, Scavenging Sentinel", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hyena,Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Yannik enters, exile another creature you control until Yannik leaves the battlefield. When you do, distribute X +1/+1 counters among any number of target creatures, where X is the exiled creature's power.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_3, "When Yannik enters, exile another creature you control until Yannik leaves the battlefield. When you do, distribute X +1/+1 counters among any number of target creatures, where X is the exiled creature's power.").targets(REFLEXIVE_3_TARGETS).divide(Divide { total: planned_value!("target with a divided amount read as chosen", 1u32 => Divide::read(|q: &dyn Query, asker: &Asker| (q.event_subject().object().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as u32)), of_x: None })], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
