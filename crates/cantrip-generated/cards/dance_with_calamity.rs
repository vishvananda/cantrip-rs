// Dance with Calamity
// status: ready
// Shuffle your library. As many times as you choose, you may exile the top card of your library. If the total mana value of the cards exiled this way is 13 or less, you may cast any number of spells from among those cards without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
loop { let mut again_1 = false;
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_2, after_2, before_2, picked_2): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let selected_4_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::Exile, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
let objects_3 = { let q: &dyn Query = g; selected_4.clone() }; let moved_3 = g.exile_all(&objects_3);
let paid_3 = moved_3.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_3, { let q: &dyn Query = g; (moved_3).clone() }, { let q: &dyn Query = g; (objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_4).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
again_1 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_6.extend(after_2.clone());
hoisted_7.extend(picked_2.clone());
hoisted_8.extend({ let q: &dyn Query = g; before_2.clone() });
if !again_1 { break; } }
g.end_sentence();
if { let q: &dyn Query = g; ((q.exiled_with(q.linked(item))).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>()) <= (13i64) } {
let caster_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_9: Vec<ObjectId> = { let q: &dyn Query = g; (q.exiled_with(q.linked(item))).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_9: u32 = 4294967295; let mut played_9: Vec<ObjectId> = Vec::new();
while left_9 > 0 {
let mut open_9: Vec<ObjectId> = Vec::new(); for &c in &pool_9 { let ways = g.cast_ways_during_resolution_by(c, caster_9); if ways.iter().any(|w| true) { open_9.push(c); } }
let picked_9 = if open_9.is_empty() { None } else if open_9.len() > 1 || (true && (!false || pool_9.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_9, asker.source, Purpose::CastFree, pool_9.clone(), open_9.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_9, open_9[0], Purpose::CastFree)? { Some(open_9[0]) } else { None };
let Some(card_9) = picked_9 else { break; };
let ways_9: Vec<CastWay> = g.cast_ways_during_resolution_by(card_9, caster_9).into_iter().filter(|w| true).collect();
let done_9 = if ways_9.is_empty() { g.play_land_during_resolution(card_9)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_9, card_9, ways_9)?.map(|w| w.way); g.cast_during_resolution_as(card_9, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_9), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_9 { left_9 -= 1; played_9.push(g.state().obj(card_9).became.unwrap_or(card_9)); } else if 4294967295i64 == 1 { break; } if done_9 || !(true && true) { pool_9.retain(|&c| c != card_9); }
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Dance with Calamity", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
