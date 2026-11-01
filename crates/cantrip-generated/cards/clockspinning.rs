// Clockspinning
// status: planned
// requires: instruction ChooseOne
// requires: modes implied by the target chosen
// Buyback {3} (You may pay an additional {3} as you cast this spell. If you do, put this card into your hand as it resolves.)
// Choose a counter on target permanent or suspended card. Remove that counter from that permanent or card or put another of those counters on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
static ABILITY_1_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_0_target_0)];
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_0, asker.source, vec![Label::Card(card_0, 0), Label::Card(card_0, 1)])?) {
0 => {
let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
let player_1_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; for object in objects_1_0 { if g.remove_a_counter_from_among(player_1_0, asker.source, &[object])? { removed_1 += 1; } }
g.end_sentence();
}
_ => {
let from_2: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let to_2: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let kinds_2: Vec<CounterKind> = CounterKind::ALL.into_iter().filter(|&kind| true && from_2.iter().any(|&o| g.obj(o).counter_array()[kind as usize] > 0)).collect();
if !kinds_2.is_empty() { let kind = kinds_2[cantrip_mechanics::choose::one_of(g, asker.controller, asker.source, kinds_2.iter().map(|&k| Label::Counter(k)).collect())? as usize]; let puts: Vec<(ObjectId, CounterKind, i16)> = to_2.iter().copied().map(|o| (o, kind, 1i16)).collect(); g.add_counters_together(&puts); }
g.end_sentence();
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Exile) && !q.state().obj(id).token && ((q.state().obj(id).zone.is(ZoneKind::Exile) && q.state().obj(id).counters(CounterKind::Time) > 0 && q.card(q.state().obj(id).card).suspend.is_some()))) }
static ABILITY_1_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_1_target_0)];
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let decider_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_3, asker.source, vec![Label::Card(card_3, 0), Label::Card(card_3, 1)])?) {
0 => {
let mut removed_4: i64 = 0;
let objects_4_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
let player_4_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; for object in objects_4_0 { if g.remove_a_counter_from_among(player_4_0, asker.source, &[object])? { removed_4 += 1; } }
g.end_sentence();
}
_ => {
let from_5: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let to_5: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let kinds_5: Vec<CounterKind> = CounterKind::ALL.into_iter().filter(|&kind| true && from_5.iter().any(|&o| g.obj(o).counter_array()[kind as usize] > 0)).collect();
if !kinds_5.is_empty() { let kind = kinds_5[cantrip_mechanics::choose::one_of(g, asker.controller, asker.source, kinds_5.iter().map(|&k| Label::Counter(k)).collect())? as usize]; let puts: Vec<(ObjectId, CounterKind, i16)> = to_5.iter().copied().map(|o| (o, kind, 1i16)).collect(); g.add_counters_together(&puts); }
g.end_sentence();
}
}
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = planned_value!("modes implied by the target chosen", modes(&[mode(ability_1_mode_0, "").targets(ABILITY_1_MODE_0_TARGETS),mode(ability_1_mode_1, "").targets(ABILITY_1_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_1_mode_0, "").targets(ABILITY_1_MODE_0_TARGETS),mode(ability_1_mode_1, "").targets(ABILITY_1_MODE_1_TARGETS)]).choose(1, 1).implied_by_target());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Clockspinning", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[cantrip_mechanics::replacements::BUYBACK], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_1_MODES), enchant: None, kicker: kicker(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
