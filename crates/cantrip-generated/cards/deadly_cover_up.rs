// Deadly Cover-Up
// status: ready
// As an additional cost to cast this spell, you may collect evidence 6.
// Destroy all creatures. If evidence was collected, exile a card from an opponent's graveyard. Then search its owner's graveyard, hand, and library for any number of cards with that name and exile them. That player shuffles, then draws a card for each card exiled from their hand this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
if { let q: &dyn Query = g; (i64::from(q.state().obj(asker.source).paid)) >= (1i64) } {
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let (carried_5_0,) = if let Some(present_5_0) = { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().map(|o: ObjectId| q.state().obj(o).owner) } { let searcher_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_4 = { let q: &dyn Query = g; present_5_0 };
let mut found_4: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_4)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (moved_2.clone()).get(0 as usize).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_4, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_4.extend(taken); } }
{ let shown = g.state().zone(Zone::hand(owner_4)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (moved_2.clone()).get(0 as usize).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_4, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_4.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_4); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (moved_2.clone()).get(0 as usize).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; let taken = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?; found_4.extend(taken); }
(found_4.clone(),) } else { Default::default() };
let objects_6 = { let q: &dyn Query = g; carried_5_0.clone() }; let moved_6 = g.exile_all(&objects_6);
for player in { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_8 = { let q: &dyn Query = g; (((objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).zone.kind() == ZoneKind::Hand).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let players_8 = { let q: &dyn Query = g; (moved_6.clone()).get(0 as usize).copied().map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.draw(player, amount_8); }
g.end_sentence();
hoisted_9.extend(moved_2.clone());
hoisted_10.extend(moved_6.clone());
hoisted_11.extend(selected_1.clone());
hoisted_12.extend(carried_5_0.clone());
hoisted_13.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_14.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Deadly Cover-Up", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Cast without collecting", "Collect evidence 6"], optional_cost: Some(OptionalCost::new(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { collect_evidence: 6, ..Cost::NONE }))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
