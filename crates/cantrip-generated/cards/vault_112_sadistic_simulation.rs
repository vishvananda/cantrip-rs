// Vault 112: Sadistic Simulation
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I, II — Tap up to one target creature and put a stun counter on it. You get {E}{E} (two energy counters).
// III — Pay any amount of {E}. If you paid one or more {E} this way, shuffle your library, then exile that many cards from the top. You may play one of those cards without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0).between(0, 1)];
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_0);
g.end_sentence();
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::Stun, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_2, amount_2);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_1_target_0).between(0, 1)];
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_3 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_3: Vec<ObjectId> = objects_3.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_3);
g.end_sentence();
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::Stun, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_5, amount_5);
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut done_6: i64 = 0; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { done_6 += { let mut paid = 0i64; while g.state().player(player).energy() > 0 && cantrip_mechanics::effects::ask_may(g, player, asker.source, Purpose::Optional)? { g.change_energy(player, -1); paid += 1; } paid }; }
g.end_sentence();
if { let q: &dyn Query = g; ((done_6 as i64)) >= (1i64) } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take(((done_6 as i64)) as usize).collect::<Vec<ObjectId>>() }; let moved_8 = g.exile_all(&objects_8);
g.end_sentence();
let caster_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_9: Vec<ObjectId> = { let q: &dyn Query = g; (moved_8.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_9: u32 = 1; let mut played_9: Vec<ObjectId> = Vec::new();
while left_9 > 0 {
let mut open_9: Vec<ObjectId> = Vec::new(); for &c in &pool_9 { let ways = g.cast_ways_during_resolution_by(c, caster_9); if ways.iter().any(|w| true) || g.could_play_land_during_resolution(c) { open_9.push(c); } }
let picked_9 = if open_9.is_empty() { None } else if open_9.len() > 1 || (true && (!false || pool_9.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_9, asker.source, Purpose::CastFree, pool_9.clone(), open_9.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_9, open_9[0], Purpose::CastFree)? { Some(open_9[0]) } else { None };
let Some(card_9) = picked_9 else { break; };
let ways_9: Vec<CastWay> = g.cast_ways_during_resolution_by(card_9, caster_9).into_iter().filter(|w| true).collect();
let done_9 = if ways_9.is_empty() { g.play_land_during_resolution(card_9)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_9, card_9, ways_9)?.map(|w| w.way); g.cast_during_resolution_as(card_9, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_9), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_9 { left_9 -= 1; played_9.push(g.state().obj(card_9).became.unwrap_or(card_9)); } else if true { break; } if done_9 || !(true && true) { pool_9.retain(|&c| c != card_9); }
}
g.end_sentence();
hoisted_10.extend(moved_8.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Vault 112: Sadistic Simulation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II — Tap up to one target creature and put a stun counter on it. You get {E}{E} .").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "I, II — Tap up to one target creature and put a stun counter on it. You get {E}{E} .").targets(ABILITY_0_1_TARGETS),chapter(3, ability_0_2, "III — Pay any amount of {E}. If you paid one or more {E} this way, shuffle your library, then exile that many cards from the top. You may play one of those cards without paying its mana cost.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
