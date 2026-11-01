// Unexpected Results
// status: ready
// Shuffle your library, then reveal the top card. If it's a nonland card, you may cast it without paying its mana cost. If it's a land card, you may put it onto the battlefield and return Unexpected Results to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
if { let q: &dyn Query = g; (((objects_1.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| true) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (false && (!false || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::CastFree, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::CastFree)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| true).collect();
let done_2 = if ways_2.is_empty() { g.play_land_during_resolution(card_2)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_2), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else if 1i64 == 1 { break; } if done_2 || !(true && false) { pool_2.retain(|&c| c != card_2); }
}
g.end_sentence();
} else {
let decider_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_3, asker.source, Purpose::Optional)? {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let top_4 = cantrip_mechanics::top_of_library(g, player_4, 1);
let matching_4: Vec<ObjectId> = { let q: &dyn Query = g; top_4.iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect() };
let objects_5 = ({ let q: &dyn Query = g; matching_4.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_5 = { let rows_5: Vec<(ObjectId, PlayerId, bool)> = objects_5.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_5 = g.put_all_onto_battlefield_as(&rows_5)?; came_5.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_6.extend(moved_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_8 = g.return_to_hand_revealed(&objects_8);
g.end_sentence();
hoisted_9.extend(hoisted_6.clone());
hoisted_10.extend(moved_8.clone());
hoisted_11.extend({ let q: &dyn Query = g; hoisted_7.clone() });
hoisted_12.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Unexpected Results", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
