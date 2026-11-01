// Planetarium of Wan Shi Tong
// status: ready
// {1}, {T}: Scry 2.
// Whenever you scry or surveil, look at the top card of your library. You may cast that card without paying its mana cost. Do this only once each turn. (Look at the card after you scry or surveil.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Scried { player, looked, .. } = event { let subject = Target::Player(*player); let data = [*looked as u32, 0]; if Some(asker.controller) == Some(*player) && !q.history().done(asker.source, AbilityRef { card: q.chars(asker.source).card, index: 201 }, asker.controller) { return Some(Fired { subject, data }); } }
if let Event::Surveilled { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) && !q.history().done(asker.source, AbilityRef { card: q.chars(asker.source).card, index: 201 }, asker.controller) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !g.history().done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 201 }, decider_2) && cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? { g.note_done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 201 }, decider_2);
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_3 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_3, &objects_3);
g.end_sentence();
let caster_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_4: Vec<ObjectId> = { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_4: u32 = 1; let mut played_4: Vec<ObjectId> = Vec::new();
while left_4 > 0 {
let mut open_4: Vec<ObjectId> = Vec::new(); for &c in &pool_4 { let ways = g.cast_ways_during_resolution_by(c, caster_4); if ways.iter().any(|w| true) { open_4.push(c); } }
let picked_4 = if open_4.is_empty() { None } else if open_4.len() > 1 || (false && (!false || pool_4.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_4, asker.source, Purpose::CastFree, pool_4.clone(), open_4.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_4, open_4[0], Purpose::CastFree)? { Some(open_4[0]) } else { None };
let Some(card_4) = picked_4 else { break; };
let ways_4: Vec<CastWay> = g.cast_ways_during_resolution_by(card_4, caster_4).into_iter().filter(|w| true).collect();
let done_4 = if ways_4.is_empty() { g.play_land_during_resolution(card_4)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_4, card_4, ways_4)?.map(|w| w.way); g.cast_during_resolution_as(card_4, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_4), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_4 { left_4 -= 1; played_4.push(g.state().obj(card_4).became.unwrap_or(card_4)); } else if 1i64 == 1 { break; } if done_4 || !(true && false) { pool_4.retain(|&c| c != card_4); }
}
g.end_sentence();
hoisted_5.extend(objects_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Planetarium of Wan Shi Tong", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{1}, {T}: Scry 2.").targets(&[]),triggered(when_asked(EventMask(EventMask::SCRIED.0 | EventMask::SURVEILLED.0), ability_1_event), ability_1, "Whenever you scry or surveil, look at the top card of your library. You may cast that card without paying its mana cost. Do this only once each turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
