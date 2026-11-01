// Talent of the Telepath
// status: ready
// Target opponent reveals the top seven cards of their library. You may cast an instant or sorcery spell from among them without paying its mana cost. Then that player puts the rest into their graveyard.
// Spell mastery — If there are two or more instant and/or sorcery cards in your graveyard, you may cast up to two instant and/or sorcery spells from among the revealed cards instead of one.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>()).len() as i64) >= (2i64) } {
let objects_0 = { let q: &dyn Query = g; (((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((7i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let caster_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_1: Vec<ObjectId> = { let q: &dyn Query = g; ((objects_0.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_1: u32 = 2; let mut played_1: Vec<ObjectId> = Vec::new();
while left_1 > 0 {
let mut open_1: Vec<ObjectId> = Vec::new(); for &c in &pool_1 { let ways = g.cast_ways_during_resolution_by(c, caster_1); if ways.iter().any(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))) { open_1.push(c); } }
let picked_1 = if open_1.is_empty() { None } else if open_1.len() > 1 || (true && (!false || pool_1.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_1, asker.source, Purpose::CastFree, pool_1.clone(), open_1.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_1, open_1[0], Purpose::CastFree)? { Some(open_1[0]) } else { None };
let Some(card_1) = picked_1 else { break; };
let ways_1: Vec<CastWay> = g.cast_ways_during_resolution_by(card_1, caster_1).into_iter().filter(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))).collect();
let done_1 = if ways_1.is_empty() { g.play_land_during_resolution(card_1)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_1, card_1, ways_1)?.map(|w| w.way); g.cast_during_resolution_as(card_1, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_1), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_1 { left_1 -= 1; played_1.push(g.state().obj(card_1).became.unwrap_or(card_1)); } else if 2i64 == 1 { break; } if done_1 || !(true && true) { pool_1.retain(|&c| c != card_1); }
}
let objects_2 = { let q: &dyn Query = g; (objects_0.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let moved_2 = { let moves: Vec<(ObjectId, Zone)> = objects_2.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend(objects_0.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let objects_6 = { let q: &dyn Query = g; (((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((7i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_6: Vec<PlayerId> = objects_6.iter().map(|o| g.state().obj(*o).owner).collect(); owners_6.dedup(); for owner in owners_6 { let cards: Vec<ObjectId> = objects_6.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let caster_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_7: Vec<ObjectId> = { let q: &dyn Query = g; ((objects_6.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_7: u32 = 1; let mut played_7: Vec<ObjectId> = Vec::new();
while left_7 > 0 {
let mut open_7: Vec<ObjectId> = Vec::new(); for &c in &pool_7 { let ways = g.cast_ways_during_resolution_by(c, caster_7); if ways.iter().any(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))) { open_7.push(c); } }
let picked_7 = if open_7.is_empty() { None } else if open_7.len() > 1 || (true && (!false || pool_7.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_7, asker.source, Purpose::CastFree, pool_7.clone(), open_7.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_7, open_7[0], Purpose::CastFree)? { Some(open_7[0]) } else { None };
let Some(card_7) = picked_7 else { break; };
let ways_7: Vec<CastWay> = g.cast_ways_during_resolution_by(card_7, caster_7).into_iter().filter(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))).collect();
let done_7 = if ways_7.is_empty() { g.play_land_during_resolution(card_7)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_7, card_7, ways_7)?.map(|w| w.way); g.cast_during_resolution_as(card_7, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_7), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_7 { left_7 -= 1; played_7.push(g.state().obj(card_7).became.unwrap_or(card_7)); } else if 1i64 == 1 { break; } if done_7 || !(true && true) { pool_7.retain(|&c| c != card_7); }
}
let objects_8 = { let q: &dyn Query = g; (objects_6.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let moved_8 = { let moves: Vec<(ObjectId, Zone)> = objects_8.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_9.extend(moved_8.clone());
hoisted_10.extend(objects_6.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Talent of the Telepath", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
