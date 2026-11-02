// Emergent Ultimatum
// status: ready
// Search your library for up to three monocolored cards with different names and exile them. An opponent chooses one of those cards. Shuffle that card into your library. You may cast the other cards without paying their mana costs. Exile Emergent Ultimatum.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((3i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).colors.iter().count() as i64) == (1i64)).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0).with(cantrip_mechanics::select::DIFFERENT_NAMES))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let moved_1 = g.exile_all(&objects_1);
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let selected_3_player = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; moved_1.clone() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutOnTop, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };

let objects_5 = ({ let q: &dyn Query = g; selected_3.clone() }).into_iter().rev().collect::<Vec<ObjectId>>(); let moved_5 = g.put_into_library(&objects_5, LibraryPlace::Top);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let caster_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_7: Vec<ObjectId> = { let q: &dyn Query = g; (moved_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_7: u32 = 4294967295; let mut played_7: Vec<ObjectId> = Vec::new();
while left_7 > 0 {
let mut open_7: Vec<ObjectId> = Vec::new(); for &c in &pool_7 { let ways = g.cast_ways_during_resolution_by(c, caster_7); if ways.iter().any(|w| true) { open_7.push(c); } }
let picked_7 = if open_7.is_empty() { None } else if open_7.len() > 1 || (true && (!true || pool_7.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_7, asker.source, Purpose::CastFree, pool_7.clone(), open_7.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_7, open_7[0], Purpose::CastFree)? { Some(open_7[0]) } else { None };
let Some(card_7) = picked_7 else { break; };
let ways_7: Vec<CastWay> = g.cast_ways_during_resolution_by(card_7, caster_7).into_iter().filter(|w| true).collect();
let done_7 = if ways_7.is_empty() { g.play_land_during_resolution(card_7)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_7, card_7, ways_7)?.map(|w| w.way); g.cast_during_resolution_as(card_7, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_7), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_7 { left_7 -= 1; played_7.push(g.state().obj(card_7).became.unwrap_or(card_7)); } else if false { break; } if done_7 || !(true && true) { pool_7.retain(|&c| c != card_7); }
}
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_8 = g.exile_all(&objects_8);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Emergent Ultimatum", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 2, 2, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
