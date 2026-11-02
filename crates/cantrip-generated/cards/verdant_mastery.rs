// Verdant Mastery
// status: ready
// You may pay {3}{G} rather than pay this spell's mana cost.
// Search your library for up to four basic land cards and reveal them. Put one of them onto the battlefield tapped under an opponent's control if the {3}{G} cost was paid. Put two of them onto the battlefield tapped under your control and the rest into your hand. Then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_31: Vec<ObjectId> = Vec::new();
let mut hoisted_30: Vec<ObjectId> = Vec::new();
let mut hoisted_29: Vec<ObjectId> = Vec::new();
let mut hoisted_28: Vec<ObjectId> = Vec::new();
let mut hoisted_27: Vec<ObjectId> = Vec::new();
let mut hoisted_26: Vec<ObjectId> = Vec::new();
let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.way == 1 } {
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((4i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; found_0.clone() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::PutOntoBattlefield, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let player_4 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; selected_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, player_4, true)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };

let selected_6_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; found_0.clone() }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::PutOntoBattlefield, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; selected_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, true)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
let objects_8 = { let q: &dyn Query = g; found_0.clone() }; let moved_8 = g.return_to_hand_revealed(&objects_8);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_10.extend(moved_4.clone());
hoisted_11.extend(moved_7.clone());
hoisted_12.extend(moved_8.clone());
hoisted_13.extend(found_0.clone());
hoisted_14.extend(objects_1.clone());
hoisted_15.extend(selected_2.clone());
hoisted_16.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_17.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_18.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let searcher_19 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_19 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_19 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((4i64).max(0) as u32) }; let shown_19 = cantrip_mechanics::library::searched_cards(g, owner_19); let can_19: Vec<ObjectId> = { let q: &dyn Query = g; shown_19.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_19 = cantrip_mechanics::library::search(g, searcher_19, owner_19, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_19, &shown_19, can_19))?;
let objects_20 = { let q: &dyn Query = g; found_19.clone() }; let mut owners_20: Vec<PlayerId> = objects_20.iter().map(|o| g.state().obj(*o).owner).collect(); owners_20.dedup(); for owner in owners_20 { let cards: Vec<ObjectId> = objects_20.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_22_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_22_candidates = { let q: &dyn Query = g; found_19.clone() }; let selected_22_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_22 = { cantrip_mechanics::choose::select(g, selected_22_player, asker.source, Purpose::PutOntoBattlefield, selected_22_candidates, cantrip_mechanics::select::Selection::of(selected_22_count, &[]))? };
let player_23 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_23 = ({ let q: &dyn Query = g; selected_22.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_23 = { let rows_23: Vec<(ObjectId, PlayerId, bool)> = objects_23.iter().map(|&o| (o, player_23, true)).collect(); let came_23 = g.put_all_onto_battlefield_as(&rows_23)?; came_23.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
let objects_24 = { let q: &dyn Query = g; found_19.clone() }; let moved_24 = g.return_to_hand_revealed(&objects_24);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_26.extend(moved_23.clone());
hoisted_27.extend(moved_24.clone());
hoisted_28.extend(found_19.clone());
hoisted_29.extend(objects_20.clone());
hoisted_30.extend({ let q: &dyn Query = g; objects_23.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_31.extend({ let q: &dyn Query = g; objects_24.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Verdant Mastery", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Hand, "You may pay {3}{G} rather than pay this spell's mana cost.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
