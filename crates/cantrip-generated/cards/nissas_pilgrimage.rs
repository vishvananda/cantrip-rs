// Nissa's Pilgrimage
// status: ready
// Search your library for up to two basic Forest cards, reveal those cards, and put one onto the battlefield tapped and the rest into your hand. Then shuffle.
// Spell mastery — If there are two or more instant and/or sorcery cards in your graveyard, search your library for up to three basic Forest cards instead of two.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_25: Vec<ObjectId> = Vec::new();
let mut hoisted_24: Vec<ObjectId> = Vec::new();
let mut hoisted_23: Vec<ObjectId> = Vec::new();
let mut hoisted_22: Vec<ObjectId> = Vec::new();
let mut hoisted_21: Vec<ObjectId> = Vec::new();
let mut hoisted_20: Vec<ObjectId> = Vec::new();
let mut hoisted_19: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>()).len() as i64) >= (2i64) } {
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((3i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest)))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; found_0.clone() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::PutOntoBattlefield, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; selected_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, true)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
let objects_4 = { let q: &dyn Query = g; found_0.clone() }; let moved_4 = g.return_to_hand_revealed(&objects_4);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_6.extend(moved_3.clone());
hoisted_7.extend(moved_4.clone());
hoisted_8.extend(found_0.clone());
hoisted_9.extend(objects_1.clone());
hoisted_10.extend(selected_2.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_12.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let searcher_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_13 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((2i64).max(0) as u32) }; let shown_13 = cantrip_mechanics::library::searched_cards(g, owner_13); let can_13: Vec<ObjectId> = { let q: &dyn Query = g; shown_13.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Forest)))).collect() }; let found_13 = cantrip_mechanics::library::search(g, searcher_13, owner_13, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_13, &shown_13, can_13))?;
let objects_14 = { let q: &dyn Query = g; found_13.clone() }; let mut owners_14: Vec<PlayerId> = objects_14.iter().map(|o| g.state().obj(*o).owner).collect(); owners_14.dedup(); for owner in owners_14 { let cards: Vec<ObjectId> = objects_14.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_15_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_15_candidates = { let q: &dyn Query = g; found_13.clone() }; let selected_15_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_15 = { cantrip_mechanics::choose::select(g, selected_15_player, asker.source, Purpose::PutOntoBattlefield, selected_15_candidates, cantrip_mechanics::select::Selection::of(selected_15_count, &[]))? };
let player_16 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_16 = ({ let q: &dyn Query = g; selected_15.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_16 = { let rows_16: Vec<(ObjectId, PlayerId, bool)> = objects_16.iter().map(|&o| (o, player_16, true)).collect(); let came_16 = g.put_all_onto_battlefield_as(&rows_16)?; came_16.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
let objects_17 = { let q: &dyn Query = g; found_13.clone() }; let moved_17 = g.return_to_hand_revealed(&objects_17);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_19.extend(moved_16.clone());
hoisted_20.extend(moved_17.clone());
hoisted_21.extend(found_13.clone());
hoisted_22.extend(objects_14.clone());
hoisted_23.extend(selected_15.clone());
hoisted_24.extend({ let q: &dyn Query = g; objects_16.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_25.extend({ let q: &dyn Query = g; objects_17.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Nissa's Pilgrimage", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
