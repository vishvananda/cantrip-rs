// Ensnared by the Mara
// status: planned
// requires: villainous choice
// Each opponent faces a villainous choice — They exile cards from the top of their library until they exile a nonland card, then you may cast that card without paying its mana cost, or that player exiles the top four cards of their library and Ensnared by the Mara deals damage equal to the total mana value of those exiled cards to that player.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_13: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_13: Vec<ObjectId> = Vec::new();
let mut each_14: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_14: Vec<ObjectId> = Vec::new();
let mut each_15: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_15: Vec<ObjectId> = Vec::new();
let mut each_16: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_16: Vec<ObjectId> = Vec::new();
let mut each_17: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_17: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("villainous-choice" => cantrip_mechanics::choose::villainous(g, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }, asker.source, &[" Return the chosen permanents to their owners' hands. ", "Return the chosen permanents to their owners' hands. "])?);
let decider_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let offered_2: Vec<u8> = [(0u8, true), (1u8, !{ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }.is_empty())].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_2 = item.ability.card; let picked_2 = if offered_2.len() > 1 { offered_2[cantrip_mechanics::choose::one_of(g, decider_2, asker.source, offered_2.iter().map(|&option| Label::Card(card_2, option)).collect())? as usize] } else { offered_2.first().copied().unwrap_or(u8::MAX) }; match picked_2 {
0 => {
let owner_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let count_3 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_3, rest_3) = cantrip_mechanics::library::reveal_until_n(g, owner_3, count_3, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_3 = found_3.len() + rest_3.len(); let objects_3: Vec<ObjectId> = g.state().zone(Zone::library(owner_3)).iter().copied().take(shown_3).collect();
let objects_4 = { let q: &dyn Query = g; objects_3.clone() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
let caster_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_5: Vec<ObjectId> = { let q: &dyn Query = g; (((moved_4.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_5: u32 = 1; let mut played_5: Vec<ObjectId> = Vec::new();
while left_5 > 0 {
let mut open_5: Vec<ObjectId> = Vec::new(); for &c in &pool_5 { let ways = g.cast_ways_during_resolution_by(c, caster_5); if ways.iter().any(|w| true) { open_5.push(c); } }
let picked_5 = if open_5.is_empty() { None } else if open_5.len() > 1 || (false && (!false || pool_5.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_5, asker.source, Purpose::CastFree, pool_5.clone(), open_5.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_5, open_5[0], Purpose::CastFree)? { Some(open_5[0]) } else { None };
let Some(card_5) = picked_5 else { break; };
let ways_5: Vec<CastWay> = g.cast_ways_during_resolution_by(card_5, caster_5).into_iter().filter(|w| true).collect();
let done_5 = if ways_5.is_empty() { g.play_land_during_resolution(card_5)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_5, card_5, ways_5)?.map(|w| w.way); g.cast_during_resolution_as(card_5, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_5), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_5 { left_5 -= 1; played_5.push(g.state().obj(card_5).became.unwrap_or(card_5)); } else if true { break; } if done_5 || !(true && false) { pool_5.retain(|&c| c != card_5); }
}
g.end_sentence();
hoisted_6.extend(moved_4.clone());
hoisted_7.extend(objects_3.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
1 => {
let objects_9 = { let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_9 = g.exile_all(&objects_9);
g.end_sentence();
let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let amount_10 = { let q: &dyn Query = g; ((((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_9.clone()).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>()).clamp(0, 65535) as u16 }; let recipients_10 = { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_10 { let batch: Vec<(ObjectId, Target, u16)> = recipients_10.iter().map(|t| (source, *t, amount_10)).collect(); g.damage_together(&batch); }
g.end_sentence();
hoisted_11.extend(moved_9.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
_ => {}
}
 each_13[index_0] = hoisted_7.clone(); all_13.extend(each_13[index_0].iter().copied());
each_14[index_0] = hoisted_11.clone(); all_14.extend(each_14[index_0].iter().copied());
each_15[index_0] = hoisted_6.clone(); all_15.extend(each_15[index_0].iter().copied());
each_16[index_0] = { let q: &dyn Query = g; hoisted_12.clone() }; all_16.extend(each_16[index_0].iter().copied());
each_17[index_0] = { let q: &dyn Query = g; hoisted_8.clone() }; all_17.extend(each_17[index_0].iter().copied());
   } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ensnared by the Mara", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["They exile cards from the top of their library until they exile a nonland card, then you may cast that card without paying its mana cost", "That player exiles the top four cards of their library and Ensnared by the Mara deals damage equal to the total mana value of those exiled cards to that player"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
