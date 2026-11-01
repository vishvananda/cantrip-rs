// Beseech the Mirror
// status: ready
// Bargain (You may sacrifice an artifact, enchantment, or token as you cast this spell.)
// Search your library for a card, exile it face down, then shuffle. If this spell was bargained, you may cast the exiled card without paying its mana cost if that spell's mana value is 4 or less. Put the exiled card into your hand if it wasn't cast this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| true).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let by_1 = None; let moved_1 = g.exile_face_down(&objects_1, by_1);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).paid > 0) } {
let caster_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_3: Vec<ObjectId> = { let q: &dyn Query = g; ((moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_3: u32 = 1; let mut played_3: Vec<ObjectId> = Vec::new();
let limit_3_0: i64 = i64::from({ let q: &dyn Query = g; 4i64 });
while left_3 > 0 {
let mut open_3: Vec<ObjectId> = Vec::new(); for &c in &pool_3 { let ways = g.cast_ways_during_resolution_by(c, caster_3); if ways.iter().any(|w| (i64::from(w.mana_value)) <= (limit_3_0)) { open_3.push(c); } }
let picked_3 = if open_3.is_empty() { None } else if open_3.len() > 1 || (false && (!false || pool_3.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_3, asker.source, Purpose::CastFree, pool_3.clone(), open_3.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_3, open_3[0], Purpose::CastFree)? { Some(open_3[0]) } else { None };
let Some(card_3) = picked_3 else { break; };
let ways_3: Vec<CastWay> = g.cast_ways_during_resolution_by(card_3, caster_3).into_iter().filter(|w| (i64::from(w.mana_value)) <= (limit_3_0)).collect();
let done_3 = if ways_3.is_empty() { g.play_land_during_resolution(card_3)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_3, card_3, ways_3)?.map(|w| w.way); g.cast_during_resolution_as(card_3, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_3), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_3 { left_3 -= 1; played_3.push(g.state().obj(card_3).became.unwrap_or(card_3)); } else if 1i64 == 1 { break; } if done_3 || !(true && false) { pool_3.retain(|&c| c != card_3); }
}
g.end_sentence();
}
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; ((moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_4 = g.return_to_hand(&objects_4);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Beseech the Mirror", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: cantrip_mechanics::bargain::LABELS, optional_cost: Some(cantrip_mechanics::bargain::OPTIONAL_COST), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
