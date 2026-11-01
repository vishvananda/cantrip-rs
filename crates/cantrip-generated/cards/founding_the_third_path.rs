// Founding the Third Path
// status: planned
// requires: engine: no ReadAhead
// requires: numeric expression AnyOf
// Read ahead (Choose a chapter and start with that many lore counters. Add one after your draw step. Skipped chapters don't trigger. Sacrifice after III.)
// I — You may cast an instant or sorcery spell with mana value 1 or 2 from your hand without paying its mana cost.
// II — Target player mills four cards.
// III — Exile target instant or sorcery card from your graveyard. Copy it. You may cast the copy.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let caster_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = 1; let mut played_0: Vec<ObjectId> = Vec::new();
let limit_0_0: i64 = i64::from({ let q: &dyn Query = g; planned_value!("numeric expression AnyOf", 0i64 => cantrip_mechanics::numbers::AnyOf(&[1i64, 2i64])) });
while left_0 > 0 {
let mut open_0: Vec<ObjectId> = Vec::new(); for &c in &pool_0 { let ways = g.cast_ways_during_resolution_by(c, caster_0); if ways.iter().any(|w| ((g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery)) && (i64::from(w.mana_value)) == (limit_0_0))) { open_0.push(c); } }
let picked_0 = if open_0.is_empty() { None } else if open_0.len() > 1 || (true && (!false || pool_0.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_0, asker.source, Purpose::CastFree, pool_0.clone(), open_0.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_0, open_0[0], Purpose::CastFree)? { Some(open_0[0]) } else { None };
let Some(card_0) = picked_0 else { break; };
let ways_0: Vec<CastWay> = g.cast_ways_during_resolution_by(card_0, caster_0).into_iter().filter(|w| ((g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery)) && (i64::from(w.mana_value)) == (limit_0_0))).collect();
let done_0 = if ways_0.is_empty() { g.play_land_during_resolution(card_0)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_0, card_0, ways_0)?.map(|w| w.way); g.cast_during_resolution_as(card_0, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_0), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_0 { left_0 -= 1; played_0.push(g.state().obj(card_0).became.unwrap_or(card_0)); } else if 1i64 == 1 { break; } if done_0 || !(true && true) { pool_0.retain(|&c| c != card_0); }
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_1_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_1_target_0)];
fn ability_1_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_1 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_1_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_2_target_0)];
fn ability_1_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_3 = { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_3: Vec<ObjectId> = Vec::new(); for _ in 0..times_3 { for &card in &originals_3 { copies_3.push(g.create_copy_in_exile(card, player_3)); } }
g.end_sentence();
let caster_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_4: Vec<ObjectId> = { let q: &dyn Query = g; (copies_3.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_4: u32 = 1; let mut played_4: Vec<ObjectId> = Vec::new();
while left_4 > 0 {
let mut open_4: Vec<ObjectId> = Vec::new(); for &c in &pool_4 { let ways = g.cast_ways_during_resolution_by(c, caster_4); if ways.iter().any(|w| true) { open_4.push(c); } }
let picked_4 = if open_4.is_empty() { None } else if open_4.len() > 1 || (true && (!true || pool_4.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_4, asker.source, Purpose::Optional, pool_4.clone(), open_4.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_4, open_4[0], Purpose::Optional)? { Some(open_4[0]) } else { None };
let Some(card_4) = picked_4 else { break; };
let ways_4: Vec<CastWay> = g.cast_ways_during_resolution_by(card_4, caster_4).into_iter().filter(|w| true).collect();
let done_4 = if ways_4.is_empty() { g.play_land_during_resolution(card_4)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_4, card_4, ways_4)?.map(|w| w.way); g.cast_during_resolution_as(card_4, CastTerms { instead: None, way, caster: Some(caster_4), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_4 { left_4 -= 1; played_4.push(g.state().obj(card_4).became.unwrap_or(card_4)); } else if 1i64 == 1 { break; } if done_4 || !(true && true) { pool_4.retain(|&c| c != card_4); }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-read-ahead" => enters: cantrip_mechanics::read_ahead::enters);

pub static CARD: CardDef = CardDef { name: "Founding the Third Path", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_1_0, "I — You may cast an instant or sorcery spell with mana value 1 or 2 from your hand without paying its mana cost.").targets(&[]),chapter(2, ability_1_1, "II — Target player mills four cards.").targets(ABILITY_1_1_TARGETS),chapter(3, ability_1_2, "III — Exile target instant or sorcery card from your graveyard. Copy it. You may cast the copy.").targets(ABILITY_1_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
