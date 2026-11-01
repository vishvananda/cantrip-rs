// The Five Doctors
// status: ready
// Kicker {5} (You may pay an additional {5} as you cast this spell.)
// Search your library and/or graveyard for up to five Doctor cards, reveal them, and put them into your hand. If you search your library this way, shuffle. If this spell was kicked, put those cards onto the battlefield instead of putting them into your hand.
#[allow(unused_imports)] use cantrip_api::*;
static ASK_0: Question = Question::new("SearchLibrary", "search your library as well as your graveyard?", "Then shuffle. If you don't, you search only your graveyard", "Search both", "Graveyard only");
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((5i64).max(0) as u32) }; let zone_0 = Zone::graveyard(searcher_0); let mut shown_0 = cantrip_mechanics::library::searched_cards(g, searcher_0); shown_0.extend(g.state().zone(zone_0).iter().copied()); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Doctor))).collect() }; let (found_0, searched_0) = cantrip_mechanics::library::search_zone_and_or_library(g, searcher_0, asker.source, zone_0, SearchDestination::Battlefield, Purpose::PutOntoBattlefield, &ASK_0, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; found_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
if { let q: &dyn Query = g; searched_0 } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static ASK_3: Question = Question::new("SearchLibrary", "search your library as well as your graveyard?", "Then shuffle. If you don't, you search only your graveyard", "Search both", "Graveyard only");
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let rule_count_3 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((5i64).max(0) as u32) }; let zone_3 = Zone::graveyard(searcher_3); let mut shown_3 = cantrip_mechanics::library::searched_cards(g, searcher_3); shown_3.extend(g.state().zone(zone_3).iter().copied()); let can_3: Vec<ObjectId> = { let q: &dyn Query = g; shown_3.iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Doctor))).collect() }; let (found_3, searched_3) = cantrip_mechanics::library::search_zone_and_or_library(g, searcher_3, asker.source, zone_3, SearchDestination::Hand, Purpose::PutIntoHand, &ASK_3, cantrip_mechanics::select::Selection::listed(rule_count_3, &shown_3, can_3))?;
let objects_4 = { let q: &dyn Query = g; found_3.clone() }; let mut owners_4: Vec<PlayerId> = objects_4.iter().map(|o| g.state().obj(*o).owner).collect(); owners_4.dedup(); for owner in owners_4 { let cards: Vec<ObjectId> = objects_4.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_5 = { let q: &dyn Query = g; found_3.clone() }; let moved_5 = g.return_to_hand_revealed(&objects_5);
if { let q: &dyn Query = g; searched_3 } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.kicked > 0 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }

pub static CARD: CardDef = CardDef { name: "The Five Doctors", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: kicker(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
