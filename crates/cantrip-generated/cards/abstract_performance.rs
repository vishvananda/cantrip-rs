// Abstract Performance
// status: planned
// requires: move arrival flags
// requires: operation ChoosePile
// requires: operation Move
// Exile the top four cards of your library in a face-down pile, then exile the top four cards of your library in a face-up pile. An opponent chooses one of those piles. Put that pile into your graveyard. Look at the cards in the other pile. You may cast a spell from among them without paying its mana cost. Put the rest into your hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0: Vec<ObjectId> = planned_value!("operation Move", Vec::<ObjectId>::new() => cantrip_mechanics::enter::in_state(g, asker, &objects_0, Destination::Exile, Arrival { tapped: false, counters: &[], states: &[ArrivalState::ExilePile { shuffled: false }] }, true)?);
g.end_sentence();
let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_1: Vec<ObjectId> = planned_value!("move arrival flags", Vec::<ObjectId>::new() => cantrip_mechanics::enter::in_state(g, asker, &objects_1, Destination::Exile, Arrival { tapped: false, counters: &[], states: &[ArrivalState::ExilePile { shuffled: false }] }, false)?);
g.end_sentence();
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
planned!("operation-choose-pile" => cantrip_mechanics::choose::of_piles(g, { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }, asker.source, None, false, &[], None, None)?);

g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_2)).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_4 = { let moves: Vec<(ObjectId, Zone)> = objects_4.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_5 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !((moved_4.clone()).contains(&id))).collect::<Vec<ObjectId>>() }; g.look_at(player_5, &objects_5);
g.end_sentence();
let caster_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_6: Vec<ObjectId> = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !((objects_5.clone()).contains(&id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_6: u32 = 1; let mut played_6: Vec<ObjectId> = Vec::new();
while left_6 > 0 {
let mut open_6: Vec<ObjectId> = Vec::new(); for &c in &pool_6 { let ways = g.cast_ways_during_resolution_by(c, caster_6); if ways.iter().any(|w| true) { open_6.push(c); } }
let picked_6 = if open_6.is_empty() { None } else if open_6.len() > 1 || (true && (!false || pool_6.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_6, asker.source, Purpose::CastFree, pool_6.clone(), open_6.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_6, open_6[0], Purpose::CastFree)? { Some(open_6[0]) } else { None };
let Some(card_6) = picked_6 else { break; };
let ways_6: Vec<CastWay> = g.cast_ways_during_resolution_by(card_6, caster_6).into_iter().filter(|w| true).collect();
let done_6 = if ways_6.is_empty() { g.play_land_during_resolution(card_6)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_6, card_6, ways_6)?.map(|w| w.way); g.cast_during_resolution_as(card_6, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_6), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_6 { left_6 -= 1; played_6.push(g.state().obj(card_6).became.unwrap_or(card_6)); } else if true { break; } if done_6 || !(true && true) { pool_6.retain(|&c| c != card_6); }
}
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !((objects_5.clone()).contains(&id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_7 = g.return_to_hand(&objects_7);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Abstract Performance", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
