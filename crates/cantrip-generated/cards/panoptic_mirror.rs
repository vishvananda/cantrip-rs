// Panoptic Mirror
// status: planned
// requires: earlier objects CopiedSpell
// Imprint — {X}, {T}: You may exile an instant or sorcery card with mana value X from your hand.
// At the beginning of your upkeep, you may copy a card exiled with this artifact. If you do, you may cast the copy without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (i64::from(q.mana_value(id))) == (i64::from(item.x)))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Exile, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
if !selected_0.is_empty() { 
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let moved_1 = { let by = g.linked(item); g.exile_with(&objects_1, by, false) };
g.end_sentence();
hoisted_2.extend(moved_1.clone());
hoisted_3.extend(selected_0.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_5 = cantrip_mechanics::effects::ask_may(g, payer_5, asker.source, Purpose::Optional)?; if paid_5 {
let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_6 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>() }; let times_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_6: Vec<ObjectId> = Vec::new(); for _ in 0..times_6 { for &card in &originals_6 { copies_6.push(g.create_copy_in_exile(card, player_6)); } }
g.end_sentence();
hoisted_7.extend(copies_6.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
let caster_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_8: Vec<ObjectId> = { let q: &dyn Query = g; (planned_value!("earlier objects CopiedSpell", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::CopiedSpell))).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_8: u32 = 1; let mut played_8: Vec<ObjectId> = Vec::new();
while left_8 > 0 {
let mut open_8: Vec<ObjectId> = Vec::new(); for &c in &pool_8 { let ways = g.cast_ways_during_resolution_by(c, caster_8); if ways.iter().any(|w| true) { open_8.push(c); } }
let picked_8 = if open_8.is_empty() { None } else if open_8.len() > 1 || (true && (!false || pool_8.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_8, asker.source, Purpose::CastFree, pool_8.clone(), open_8.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_8, open_8[0], Purpose::CastFree)? { Some(open_8[0]) } else { None };
let Some(card_8) = picked_8 else { break; };
let ways_8: Vec<CastWay> = g.cast_ways_during_resolution_by(card_8, caster_8).into_iter().filter(|w| true).collect();
let done_8 = if ways_8.is_empty() { g.play_land_during_resolution(card_8)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_8, card_8, ways_8)?.map(|w| w.way); g.cast_during_resolution_as(card_8, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_8), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_8 { left_8 -= 1; played_8.push(g.state().obj(card_8).became.unwrap_or(card_8)); } else if 1i64 == 1 { break; } if done_8 || !(true && true) { pool_8.retain(|&c| c != card_8); }
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Panoptic Mirror", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "Imprint — {X}, {T}: You may exile an instant or sorcery card with mana value X from your hand.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, you may copy a card exiled with this artifact. If you do, you may cast the copy without paying its mana cost.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
