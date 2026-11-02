// Maulfist Revolutionary
// status: ready
// Trample
// When this creature enters or dies, for each kind of counter on target permanent or player, give that permanent or player another counter of that kind.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
#[allow(unused_variables)] fn ability_1_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { ability_1_mode_0_target_0(q, asker, t) || ability_1_mode_1_target_0(q, asker, t) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let from_0: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let to_0: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let kinds_0: Vec<CounterKind> = CounterKind::ALL.into_iter().filter(|&kind| true && from_0.iter().any(|&o| g.obj(o).counter_array()[kind as usize] > 0)).collect();
let puts: Vec<(ObjectId, CounterKind, i16)> = to_0.iter().flat_map(|&o| kinds_0.iter().map(move |&kind| (o, kind, 1i16)).collect::<Vec<_>>()).collect(); g.add_counters_together(&puts);
g.end_sentence();
 Ok(()) }
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { let (poison, energy, experience) = { let counters = g.state().player(player); (u32::from(counters.poison()), u32::from(counters.energy()), u32::from(counters.experience())) }; let n = poison; if n > 0 { g.give_poison(player, 1u32); } let n = energy; if n > 0 { g.change_energy(player, 1i32); } let n = experience; if n > 0 { g.give_experience(player, 1u32); } }
g.end_sentence();
 Ok(()) }
fn ability_1_implied(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let chosen = (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == 0).map(|i| g.state().target(item, i)).next().unwrap_or(Target::None);
if ability_1_mode_0_target_0(g, asker, chosen) { return ability_1_mode_0(g, item); }
ability_1_mode_1(g, item) }
#[allow(unused_variables)] fn ability_2_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
#[allow(unused_variables)] fn ability_2_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { ability_2_mode_0_target_0(q, asker, t) || ability_2_mode_1_target_0(q, asker, t) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let from_2: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let to_2: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let kinds_2: Vec<CounterKind> = CounterKind::ALL.into_iter().filter(|&kind| true && from_2.iter().any(|&o| g.obj(o).counter_array()[kind as usize] > 0)).collect();
let puts: Vec<(ObjectId, CounterKind, i16)> = to_2.iter().flat_map(|&o| kinds_2.iter().map(move |&kind| (o, kind, 1i16)).collect::<Vec<_>>()).collect(); g.add_counters_together(&puts);
g.end_sentence();
 Ok(()) }
fn ability_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { let (poison, energy, experience) = { let counters = g.state().player(player); (u32::from(counters.poison()), u32::from(counters.energy()), u32::from(counters.experience())) }; let n = poison; if n > 0 { g.give_poison(player, 1u32); } let n = energy; if n > 0 { g.change_energy(player, 1i32); } let n = experience; if n > 0 { g.give_experience(player, 1u32); } }
g.end_sentence();
 Ok(()) }
fn ability_2_implied(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); let chosen = (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == 0).map(|i| g.state().target(item, i)).next().unwrap_or(Target::None);
if ability_2_mode_0_target_0(g, asker, chosen) { return ability_2_mode_0(g, item); }
ability_2_mode_1(g, item) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Maulfist Revolutionary", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1_implied, "When this creature enters or dies, for each kind of counter on target permanent or player, give that permanent or player another counter of that kind.").targets(ABILITY_1_TARGETS),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2_implied, "When this creature enters or dies, for each kind of counter on target permanent or player, give that permanent or player another counter of that kind.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
