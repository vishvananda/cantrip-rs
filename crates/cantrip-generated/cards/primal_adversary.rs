// Primal Adversary
// status: ready
// Trample
// When this creature enters, you may pay {1}{G} any number of times. When you pay this cost one or more times, put that many +1/+1 counters on this creature, then up to that many target lands you control become 3/3 Wolf creatures with haste that are still lands.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))))) }
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_4_target_0).between(0, TargetSpec::COUNTED)];
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_5: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (i64::from(item.x)).clamp(0, 32767) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_5.push((object, CounterKind::PlusOne, amount)); } }
let had_5: Vec<_> = counters_5.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_5); let applied_5: Vec<ObjectId> = counters_5.iter().zip(had_5).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let duration_6 = Duration::Indefinite;
let kind_6_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((3i64) as i16, (3i64) as i16) };
let kind_6_1_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Wolf)) };
let kind_6_2_0 = { let q: &dyn Query = g; EffectKind::AddTypes(TypeSet::of(&[CardType::Creature])) };
let kind_6_3_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_6 { cantrip_api::instruction(g, |g| { for &object in &objects_6 { g.add_effect(source, object, kind_6_0_0, duration_6); g.add_effect(source, object, kind_6_1_0, duration_6); g.add_effect(source, object, kind_6_2_0, duration_6); g.add_effect(source, object, kind_6_3_0, duration_6);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
'optional_0: {
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times_2 = g.may_pay_times(payer_2, asker.source, asker.source, ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, cantrip_api::ANY_NUMBER_OF_TIMES, &PAY_QUESTION)?; let paid_2 = times_2 > 0; let bound_2: i64 = i64::from(times_2);
if !paid_2 { break 'optional_0; }
if paid_2 && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None, (bound_2).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Primal Adversary", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Wolf]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, you may pay {1}{G} any number of times. When you pay this cost one or more times, put that many +1/+1 counters on this creature, then up to that many target lands you control become 3/3 Wolf creatures with haste that are still lands.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "When you pay this cost one or more times, put that many +1/+1 counters on this creature, then up to that many target lands you control become 3/3 Wolf creatures with haste that are still lands.").targets(REFLEXIVE_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
