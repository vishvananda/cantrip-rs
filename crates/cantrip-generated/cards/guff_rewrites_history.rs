// Guff Rewrites History
// status: planned
// requires: per-player instruction with several steps
// requires: target cardinality UpTo
// For each player, choose target nonenchantment, nonland permanent that player controls. Those permanents' owners shuffle them into their libraries. Each player who controlled one of those permanents exiles cards from the top of their library until they exile a nonland card, then puts the rest on the bottom of their library in a random order. Each player may cast the nonland card they exiled without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn different_controllers_key(q: &dyn Query, id: ObjectId) -> PlayerId { q.controller(id) }
fn different_controllers(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let used: Vec<PlayerId> = choice.selected.iter().map(|&c| different_controllers_key(q, c)).collect(); let apart = used.iter().enumerate().all(|(i, k)| !used[..i].contains(k)); Verdict { complete: apart, addable: Bits::of(choice.candidates.len(), |i| !used.contains(&different_controllers_key(q, choice.candidates[i]))) } }
fn largest_different_controllers(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { cantrip_api::select::largest_distinct(choice, |c| different_controllers_key(q, c)) }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (((!(q.chars(id).is(CardType::Enchantment)) && !(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))) && true))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, planned_value!("target cardinality UpTo", 1u32 => TargetSpec::read_as_chosen(|q: &dyn Query, asker: &Asker| ((q.state().apnap().collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32))).together(cantrip_api::select::set_part(&(different_controllers as SelectFn), &(largest_different_controllers as cantrip_api::select::LargestFn), 0, "controlled by different players"))];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.put_into_library(&objects_0, LibraryPlace::ShuffledIn);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let owner_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_2, rest_2) = cantrip_mechanics::library::reveal_until_n(g, owner_2, count_2, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_2 = found_2.len() + rest_2.len(); let objects_2: Vec<ObjectId> = g.state().zone(Zone::library(owner_2)).iter().copied().take(shown_2).collect();
 each_3[index_1] = objects_2.clone(); all_3.extend(each_3[index_1].iter().copied());
each_4[index_1] = found_2.clone(); all_4.extend(each_4[index_1].iter().copied());
   } 
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; all_3.clone() }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
let objects_7 = { let q: &dyn Query = g; ((moved_5.clone()).into_iter().filter(|&id| !(!(q.chars(id).is(CardType::Land)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_7 = g.put_into_library(&objects_7, LibraryPlace::BottomInRandomOrder);
g.end_sentence();
let mut players_8: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { let caster_9 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let mut pool_9: Vec<ObjectId> = { let q: &dyn Query = g; (((moved_5.clone()).into_iter().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_9: u32 = 1; let mut played_9: Vec<ObjectId> = Vec::new();
while left_9 > 0 {
let mut open_9: Vec<ObjectId> = Vec::new(); for &c in &pool_9 { let ways = g.cast_ways_during_resolution_by(c, caster_9); if ways.iter().any(|w| true) { open_9.push(c); } }
let picked_9 = if open_9.is_empty() { None } else if open_9.len() > 1 || (false && (!false || pool_9.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_9, asker.source, Purpose::CastFree, pool_9.clone(), open_9.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_9, open_9[0], Purpose::CastFree)? { Some(open_9[0]) } else { None };
let Some(card_9) = picked_9 else { break; };
let ways_9: Vec<CastWay> = g.cast_ways_during_resolution_by(card_9, caster_9).into_iter().filter(|w| true).collect();
let done_9 = if ways_9.is_empty() { g.play_land_during_resolution(card_9)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_9, card_9, ways_9)?.map(|w| w.way); g.cast_during_resolution_as(card_9, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_9), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_9 { left_9 -= 1; played_9.push(g.state().obj(card_9).became.unwrap_or(card_9)); } else if 1i64 == 1 { break; } if done_9 || !(true && false) { pool_9.retain(|&c| c != card_9); }
}
    } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Guff Rewrites History", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
