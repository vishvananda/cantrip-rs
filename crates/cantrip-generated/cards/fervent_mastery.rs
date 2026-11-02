// Fervent Mastery
// status: ready
// You may pay {2}{R}{R} rather than pay this spell's mana cost.
// If the {2}{R}{R} cost was paid, an opponent discards any number of cards, then draws that many cards.
// Search your library for up to three cards, put them into your hand, shuffle, then discard three cards at random.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.way == 1 } {
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let Some(chosen_0) = cantrip_mechanics::choose::player(g, player_0, asker.source, candidates_0, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let selected_1_player = { let q: &dyn Query = g; Some(chosen_0).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(chosen_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Discard, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let cards_2 = { let q: &dyn Query = g; selected_1.clone() }; let discarded_2 = g.discard_together(&cards_2);
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; ((discarded_2.clone()).len() as i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(chosen_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
hoisted_4.extend(discarded_2.clone());
hoisted_5.extend(selected_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; discarded_2.clone() });
}
g.end_sentence();
let searcher_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_7 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((3i64).max(0) as u32) }; let shown_7 = cantrip_mechanics::library::searched_cards(g, owner_7); let can_7: Vec<ObjectId> = { let q: &dyn Query = g; shown_7.iter().copied().filter(|&id| true).collect() }; let found_7 = cantrip_mechanics::library::search(g, searcher_7, owner_7, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_7, &shown_7, can_7))?;
let objects_8 = { let q: &dyn Query = g; found_7.clone() }; let moved_8 = g.return_to_hand(&objects_8);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_10 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let player_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
cantrip_mechanics::discard_at_random(g, player_10, amount_10 as usize);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fervent Mastery", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Hand, "You may pay {2}{R}{R} rather than pay this spell's mana cost.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
