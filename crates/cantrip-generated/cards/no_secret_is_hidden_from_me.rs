// No Secret Is Hidden from Me
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, exile cards from the top of your library until you exile a nonland card. You may cast that card without paying its mana cost. Then if you control six or more lands, repeat this process once.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut runs_0: u32 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; while runs_0 > 0 { runs_0 -= 1; let mut again_0 = false;
let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_1, rest_1) = cantrip_mechanics::library::reveal_until_n(g, owner_1, count_1, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_1 = found_1.len() + rest_1.len(); let objects_1: Vec<ObjectId> = g.state().zone(Zone::library(owner_1)).iter().copied().take(shown_1).collect();
let objects_2 = { let q: &dyn Query = g; objects_1.clone() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let caster_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_3: Vec<ObjectId> = { let q: &dyn Query = g; (((moved_2.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_3: u32 = 1; let mut played_3: Vec<ObjectId> = Vec::new();
while left_3 > 0 {
let mut open_3: Vec<ObjectId> = Vec::new(); for &c in &pool_3 { let ways = g.cast_ways_during_resolution_by(c, caster_3); if ways.iter().any(|w| true) { open_3.push(c); } }
let picked_3 = if open_3.is_empty() { None } else if cantrip_mechanics::effects::ask_may(g, caster_3, open_3[0], Purpose::CastFree)? { Some(open_3[0]) } else { None };
let Some(card_3) = picked_3 else { break; };
let ways_3: Vec<CastWay> = g.cast_ways_during_resolution_by(card_3, caster_3).into_iter().filter(|w| true).collect();
let done_3 = { let way = cantrip_mechanics::choose::cast_way(g, caster_3, card_3, ways_3)?.map(|w| w.way); g.cast_during_resolution_as(card_3, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_3), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_3 { left_3 -= 1; played_3.push(g.state().obj(card_3).became.unwrap_or(card_3)); } else { break; } pool_3.retain(|&c| c != card_3);
}
g.end_sentence();
if { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (6i64) } {
again_0 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_5.extend(moved_2.clone());
hoisted_6.extend(objects_1.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "No Secret Is Hidden from Me", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, exile cards from the top of your library until you exile a nonland card. You may cast that card without paying its mana cost. Then if you control six or more lands, repeat this process once.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
