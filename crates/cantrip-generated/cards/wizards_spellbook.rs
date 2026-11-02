// Wizard's Spellbook
// status: planned
// requires: operation PlayWith
// {T}: Exile target instant or sorcery card from a graveyard. Roll a d20. Activate only as a sorcery.
// 1—9 | Copy that card. You may cast the copy.
// 10—19 | Copy that card. You may cast the copy by paying {1} rather than paying its mana cost.
// 20 | Copy each card exiled with this artifact. You may cast any number of the copies without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = { let by = g.linked(item); g.exile_with(&objects_0, by, false) };
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_1 = i64::from(g.roll_die(player_1, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_1) >= (1i64) && (value_1) <= (9i64)) } {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_2 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_2: Vec<ObjectId> = Vec::new(); for _ in 0..times_2 { for &card in &originals_2 { copies_2.push(g.create_copy_in_exile(card, player_2)); } }
g.end_sentence();
let caster_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_3: Vec<ObjectId> = { let q: &dyn Query = g; (copies_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_3: u32 = 1; let mut played_3: Vec<ObjectId> = Vec::new();
while left_3 > 0 {
let mut open_3: Vec<ObjectId> = Vec::new(); for &c in &pool_3 { let ways = g.cast_ways_during_resolution_by(c, caster_3); if ways.iter().any(|w| true) { open_3.push(c); } }
let picked_3 = if open_3.is_empty() { None } else if open_3.len() > 1 || (true && (!true || pool_3.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_3, asker.source, Purpose::Optional, pool_3.clone(), open_3.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_3, open_3[0], Purpose::Optional)? { Some(open_3[0]) } else { None };
let Some(card_3) = picked_3 else { break; };
let ways_3: Vec<CastWay> = g.cast_ways_during_resolution_by(card_3, caster_3).into_iter().filter(|w| true).collect();
let done_3 = { let way = cantrip_mechanics::choose::cast_way(g, caster_3, card_3, ways_3)?.map(|w| w.way); g.cast_during_resolution_as(card_3, CastTerms { instead: None, way, caster: Some(caster_3), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_3 { left_3 -= 1; played_3.push(g.state().obj(card_3).became.unwrap_or(card_3)); } else { break; } if done_3 || !true { pool_3.retain(|&c| c != card_3); }
}
g.end_sentence();
hoisted_4.extend(copies_2.clone());
}
if { let q: &dyn Query = g; ((value_1) >= (10i64) && (value_1) <= (19i64)) } {
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_5 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_5: Vec<ObjectId> = Vec::new(); for _ in 0..times_5 { for &card in &originals_5 { copies_5.push(g.create_copy_in_exile(card, player_5)); } }
g.end_sentence();
let decider_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_6, asker.source, Purpose::Optional)? {
planned!("operation-play-with" => cantrip_mechanics::play::with_terms(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, CardPlay::CastSpell, { let q: &dyn Query = g; copies_5.clone() }, { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }, None, "{\"limit\":null,\"during\":null,\"cost\":{\"Alternate\":{\"kind\":\"Cost\",\"args\":{\"parts\":[{\"Mana\":[{\"Generic\":1}]}]}}},\"spend_as\":null,\"into_exile\":false,\"condition\":null,\"effects\":[],\"flash\":false,\"may_look\":false,\"then\":null,\"face_down\":null}")?);
g.end_sentence();
}
g.end_sentence();
hoisted_8.extend(copies_5.clone());
}
if { let q: &dyn Query = g; (value_1) == (20i64) } {
let player_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_9 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>() }; let times_9 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_9: Vec<ObjectId> = Vec::new(); for _ in 0..times_9 { for &card in &originals_9 { copies_9.push(g.create_copy_in_exile(card, player_9)); } }
g.end_sentence();
let caster_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_10: Vec<ObjectId> = { let q: &dyn Query = g; (copies_9.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_10: u32 = 4294967295; let mut played_10: Vec<ObjectId> = Vec::new();
while left_10 > 0 {
let mut open_10: Vec<ObjectId> = Vec::new(); for &c in &pool_10 { let ways = g.cast_ways_during_resolution_by(c, caster_10); if ways.iter().any(|w| true) { open_10.push(c); } }
let picked_10 = if open_10.is_empty() { None } else if open_10.len() > 1 || (true && (!true || pool_10.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_10, asker.source, Purpose::CastFree, pool_10.clone(), open_10.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_10, open_10[0], Purpose::CastFree)? { Some(open_10[0]) } else { None };
let Some(card_10) = picked_10 else { break; };
let ways_10: Vec<CastWay> = g.cast_ways_during_resolution_by(card_10, caster_10).into_iter().filter(|w| true).collect();
let done_10 = { let way = cantrip_mechanics::choose::cast_way(g, caster_10, card_10, ways_10)?.map(|w| w.way); g.cast_during_resolution_as(card_10, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_10), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_10 { left_10 -= 1; played_10.push(g.state().obj(card_10).became.unwrap_or(card_10)); } if done_10 || !true { pool_10.retain(|&c| c != card_10); }
}
g.end_sentence();
hoisted_11.extend(copies_9.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Wizard's Spellbook", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Book]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: Exile target instant or sorcery card from a graveyard. Roll a d20. Activate only as a sorcery.").targets(ABILITY_0_TARGETS).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
