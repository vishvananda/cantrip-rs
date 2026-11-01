// Infernal Tutor
// status: ready
// Reveal a card from your hand. Search your library for a card with the same name as that card, reveal it, put it into your hand, then shuffle.
// Hellbent — If you have no cards in hand, instead search your library for a card, put it into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) } {
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| true).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_3.extend(moved_1.clone());
hoisted_4.extend(found_0.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_6: Vec<ObjectId> = { let q: &dyn Query = g; q.state().zone(Zone::hand(player_6)).iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() };
let count_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let shown_6 = cantrip_mechanics::choose::select(g, player_6, asker.source, Purpose::Reveal, pool_6, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::AsManyAsCan(count_6), &[]))?;
g.reveal(player_6, &shown_6);
g.end_sentence();
let searcher_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_7 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_7 = cantrip_mechanics::library::searched_cards(g, owner_7); let can_7: Vec<ObjectId> = { let q: &dyn Query = g; shown_7.iter().copied().filter(|&id| (shown_6.clone()).iter().any(|&o| cantrip_mechanics::same_name(q, id, o))).collect() }; let found_7 = cantrip_mechanics::library::search(g, searcher_7, owner_7, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_7, &shown_7, can_7))?;
let objects_8 = { let q: &dyn Query = g; found_7.clone() }; let mut owners_8: Vec<PlayerId> = objects_8.iter().map(|o| g.state().obj(*o).owner).collect(); owners_8.dedup(); for owner in owners_8 { let cards: Vec<ObjectId> = objects_8.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_9 = { let q: &dyn Query = g; found_7.clone() }; let moved_9 = g.return_to_hand_revealed(&objects_9);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_11.extend(moved_9.clone());
hoisted_12.extend(shown_6.clone());
hoisted_13.extend(found_7.clone());
hoisted_14.extend(objects_8.clone());
hoisted_15.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Infernal Tutor", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
