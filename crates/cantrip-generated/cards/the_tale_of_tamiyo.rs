// The Tale of Tamiyo
// status: planned
// requires: condition SubsetMeets
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I, II, III — Mill two cards. If two cards that share a card type were milled this way, draw a card and repeat this process.
// IV — Exile any number of target instant, sorcery, and/or Tamiyo planeswalker cards from your graveyard. Copy them. You may cast any number of the copies.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_0 = false;
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition SubsetMeets", false => cantrip_mechanics::select::subset_meets(q, &(milled_1.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>(), Comparison::Equal, 2i64, &[SelectionConstraint::CommonProperty])) } {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
again_0 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_4.extend(milled_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; milled_1.clone() });
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_6 = false;
let amount_7 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_7: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_7)).collect(); let milled_7 = g.mill_together(&mills_7);
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition SubsetMeets", false => cantrip_mechanics::select::subset_meets(q, &(milled_7.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>(), Comparison::Equal, 2i64, &[SelectionConstraint::CommonProperty])) } {
let amount_8 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.draw(player, amount_8); }
g.end_sentence();
again_6 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_10.extend(milled_7.clone());
hoisted_11.extend({ let q: &dyn Query = g; milled_7.clone() });
if !again_6 { break; } }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
loop { let mut again_12 = false;
let amount_13 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let mills_13: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_13)).collect(); let milled_13 = g.mill_together(&mills_13);
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition SubsetMeets", false => cantrip_mechanics::select::subset_meets(q, &(milled_13.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>(), Comparison::Equal, 2i64, &[SelectionConstraint::CommonProperty])) } {
let amount_14 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_14 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_14 { g.draw(player, amount_14); }
g.end_sentence();
again_12 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_16.extend(milled_13.clone());
hoisted_17.extend({ let q: &dyn Query = g; milled_13.clone() });
if !again_12 { break; } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery) || (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Tamiyo)) && q.chars(id).is(CardType::Planeswalker))) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_3_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_18 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_18 = g.exile_all(&objects_18);
g.end_sentence();
let player_19 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_19 = { let q: &dyn Query = g; moved_18.clone() }; let times_19 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_19: Vec<ObjectId> = Vec::new(); for _ in 0..times_19 { for &card in &originals_19 { copies_19.push(g.create_copy_in_exile(card, player_19)); } }
g.end_sentence();
let caster_20 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_20: Vec<ObjectId> = { let q: &dyn Query = g; (copies_19.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_20: u32 = 4294967295; let mut played_20: Vec<ObjectId> = Vec::new();
while left_20 > 0 {
let mut open_20: Vec<ObjectId> = Vec::new(); for &c in &pool_20 { let ways = g.cast_ways_during_resolution_by(c, caster_20); if ways.iter().any(|w| true) { open_20.push(c); } }
let picked_20 = if open_20.is_empty() { None } else if open_20.len() > 1 || (true && (!true || pool_20.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_20, asker.source, Purpose::Optional, pool_20.clone(), open_20.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_20, open_20[0], Purpose::Optional)? { Some(open_20[0]) } else { None };
let Some(card_20) = picked_20 else { break; };
let ways_20: Vec<CastWay> = g.cast_ways_during_resolution_by(card_20, caster_20).into_iter().filter(|w| true).collect();
let done_20 = if ways_20.is_empty() { g.play_land_during_resolution(card_20)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_20, card_20, ways_20)?.map(|w| w.way); g.cast_during_resolution_as(card_20, CastTerms { instead: None, way, caster: Some(caster_20), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_20 { left_20 -= 1; played_20.push(g.state().obj(card_20).became.unwrap_or(card_20)); } else if 4294967295i64 == 1 { break; } if done_20 || !(true && true) { pool_20.retain(|&c| c != card_20); }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Tale of Tamiyo", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II, III — Mill two cards. If two cards that share a card type were milled this way, draw a card and repeat this process.").targets(&[]),chapter(2, ability_0_1, "I, II, III — Mill two cards. If two cards that share a card type were milled this way, draw a card and repeat this process.").targets(&[]),chapter(3, ability_0_2, "I, II, III — Mill two cards. If two cards that share a card type were milled this way, draw a card and repeat this process.").targets(&[]),chapter(4, ability_0_3, "IV — Exile any number of target instant, sorcery, and/or Tamiyo planeswalker cards from your graveyard. Copy them. You may cast any number of the copies.").targets(ABILITY_0_3_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
