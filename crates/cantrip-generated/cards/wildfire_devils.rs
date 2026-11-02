// Wildfire Devils
// status: ready
// When this creature enters and at the beginning of your upkeep, choose a player at random. That player exiles an instant or sorcery card from their graveyard. Copy that card. You may cast the copy without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let pool_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; if pool_1.is_empty() { return Ok(()); } let chosen_1 = pool_1[g.choose_at_random(pool_1.len())];
g.end_sentence();
let selected_2_player = { let q: &dyn Query = g; Some(chosen_1).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(chosen_1) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Exile, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let objects_3 = { let q: &dyn Query = g; selected_2.clone() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_4 = { let q: &dyn Query = g; (moved_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_4: Vec<ObjectId> = Vec::new(); for _ in 0..times_4 { for &card in &originals_4 { copies_4.push(g.create_copy_in_exile(card, player_4)); } }
g.end_sentence();
let caster_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_5: Vec<ObjectId> = { let q: &dyn Query = g; (copies_4.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_5: u32 = 1; let mut played_5: Vec<ObjectId> = Vec::new();
while left_5 > 0 {
let mut open_5: Vec<ObjectId> = Vec::new(); for &c in &pool_5 { let ways = g.cast_ways_during_resolution_by(c, caster_5); if ways.iter().any(|w| true) { open_5.push(c); } }
let picked_5 = if open_5.is_empty() { None } else if open_5.len() > 1 || (true && (!true || pool_5.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_5, asker.source, Purpose::CastFree, pool_5.clone(), open_5.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_5, open_5[0], Purpose::CastFree)? { Some(open_5[0]) } else { None };
let Some(card_5) = picked_5 else { break; };
let ways_5: Vec<CastWay> = g.cast_ways_during_resolution_by(card_5, caster_5).into_iter().filter(|w| true).collect();
let done_5 = if ways_5.is_empty() { g.play_land_during_resolution(card_5)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_5, card_5, ways_5)?.map(|w| w.way); g.cast_during_resolution_as(card_5, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_5), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_5 { left_5 -= 1; played_5.push(g.state().obj(card_5).became.unwrap_or(card_5)); } else if true { break; } if done_5 || !(true && true) { pool_5.retain(|&c| c != card_5); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wildfire Devils", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Devil]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "When this creature enters and at the beginning of your upkeep, choose a player at random. That player exiles an instant or sorcery card from their graveyard. Copy that card. You may cast the copy without paying its mana cost.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
