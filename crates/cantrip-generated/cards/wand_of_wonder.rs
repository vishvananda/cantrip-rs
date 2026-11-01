// Wand of Wonder
// status: ready
// {4}, {T}: Roll a d20. Each opponent exiles cards from the top of their library until they exile an instant or sorcery card, then shuffles the rest into their library. You may cast up to X instant and/or sorcery spells from among cards exiled this way without paying their mana costs.
// 1—9 | X is one.
// 10—19 | X is two.
// 20 | X is three.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 20).result);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let owner_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_2, rest_2) = cantrip_mechanics::library::reveal_until_n(g, owner_2, count_2, &|q: &dyn Query, id: ObjectId| (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))); let shown_2 = found_2.len() + rest_2.len(); let objects_2: Vec<ObjectId> = g.state().zone(Zone::library(owner_2)).iter().copied().take(shown_2).collect();
 each_3[index_1] = objects_2.clone(); all_3.extend(each_3[index_1].iter().copied());
each_4[index_1] = found_2.clone(); all_4.extend(each_4[index_1].iter().copied());
   } 
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; all_3.clone() }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let objects_7 = { let q: &dyn Query = g; ((moved_5.clone()).into_iter().filter(|&id| !((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_7 = g.put_into_library(&objects_7, LibraryPlace::ShuffledIn);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0) >= (1i64) && (value_0) <= (9i64)) } {
let caster_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_8: Vec<ObjectId> = { let q: &dyn Query = g; (moved_5.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_8: u32 = 1; let mut played_8: Vec<ObjectId> = Vec::new();
while left_8 > 0 {
let mut open_8: Vec<ObjectId> = Vec::new(); for &c in &pool_8 { let ways = g.cast_ways_during_resolution_by(c, caster_8); if ways.iter().any(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))) { open_8.push(c); } }
let picked_8 = if open_8.is_empty() { None } else if open_8.len() > 1 || (true && (!false || pool_8.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_8, asker.source, Purpose::CastFree, pool_8.clone(), open_8.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_8, open_8[0], Purpose::CastFree)? { Some(open_8[0]) } else { None };
let Some(card_8) = picked_8 else { break; };
let ways_8: Vec<CastWay> = g.cast_ways_during_resolution_by(card_8, caster_8).into_iter().filter(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))).collect();
let done_8 = if ways_8.is_empty() { g.play_land_during_resolution(card_8)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_8, card_8, ways_8)?.map(|w| w.way); g.cast_during_resolution_as(card_8, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_8), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_8 { left_8 -= 1; played_8.push(g.state().obj(card_8).became.unwrap_or(card_8)); } else if 1i64 == 1 { break; } if done_8 || !(true && true) { pool_8.retain(|&c| c != card_8); }
}
g.end_sentence();
}
if { let q: &dyn Query = g; ((value_0) >= (10i64) && (value_0) <= (19i64)) } {
let caster_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_9: Vec<ObjectId> = { let q: &dyn Query = g; (moved_5.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_9: u32 = 2; let mut played_9: Vec<ObjectId> = Vec::new();
while left_9 > 0 {
let mut open_9: Vec<ObjectId> = Vec::new(); for &c in &pool_9 { let ways = g.cast_ways_during_resolution_by(c, caster_9); if ways.iter().any(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))) { open_9.push(c); } }
let picked_9 = if open_9.is_empty() { None } else if open_9.len() > 1 || (true && (!false || pool_9.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_9, asker.source, Purpose::CastFree, pool_9.clone(), open_9.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_9, open_9[0], Purpose::CastFree)? { Some(open_9[0]) } else { None };
let Some(card_9) = picked_9 else { break; };
let ways_9: Vec<CastWay> = g.cast_ways_during_resolution_by(card_9, caster_9).into_iter().filter(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))).collect();
let done_9 = if ways_9.is_empty() { g.play_land_during_resolution(card_9)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_9, card_9, ways_9)?.map(|w| w.way); g.cast_during_resolution_as(card_9, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_9), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_9 { left_9 -= 1; played_9.push(g.state().obj(card_9).became.unwrap_or(card_9)); } else if 2i64 == 1 { break; } if done_9 || !(true && true) { pool_9.retain(|&c| c != card_9); }
}
g.end_sentence();
}
if { let q: &dyn Query = g; (value_0) == (20i64) } {
let caster_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_10: Vec<ObjectId> = { let q: &dyn Query = g; (moved_5.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_10: u32 = 3; let mut played_10: Vec<ObjectId> = Vec::new();
while left_10 > 0 {
let mut open_10: Vec<ObjectId> = Vec::new(); for &c in &pool_10 { let ways = g.cast_ways_during_resolution_by(c, caster_10); if ways.iter().any(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))) { open_10.push(c); } }
let picked_10 = if open_10.is_empty() { None } else if open_10.len() > 1 || (true && (!false || pool_10.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_10, asker.source, Purpose::CastFree, pool_10.clone(), open_10.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_10, open_10[0], Purpose::CastFree)? { Some(open_10[0]) } else { None };
let Some(card_10) = picked_10 else { break; };
let ways_10: Vec<CastWay> = g.cast_ways_during_resolution_by(card_10, caster_10).into_iter().filter(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))).collect();
let done_10 = if ways_10.is_empty() { g.play_land_during_resolution(card_10)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_10, card_10, ways_10)?.map(|w| w.way); g.cast_during_resolution_as(card_10, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_10), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_10 { left_10 -= 1; played_10.push(g.state().obj(card_10).became.unwrap_or(card_10)); } else if 3i64 == 1 { break; } if done_10 || !(true && true) { pool_10.retain(|&c| c != card_10); }
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wand of Wonder", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{4}, {T}: Roll a d20. Each opponent exiles cards from the top of their library until they exile an instant or sorcery card, then shuffles the rest into their library. You may cast up to X instant and/or sorcery spells from among cards exiled this way without paying their mana costs.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
