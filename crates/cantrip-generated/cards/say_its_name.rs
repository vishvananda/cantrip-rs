// Say Its Name
// status: planned
// requires: engine: no cost moves these objects
// Mill three cards. Then you may return a creature or land card from your graveyard to your hand.
// Exile this card and two other cards named Say Its Name from your graveyard: Search your graveyard, hand, and/or library for a card named Altanak, the Thrice-Called and put it onto the battlefield. If you search your library this way, shuffle. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
{
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutIntoHand, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { 
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.return_to_hand(&objects_2);
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend(selected_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
static ASK_6: Question = Question::new("SearchLibrary", "search your library as well?", "Then shuffle. If you don't, you search only the other zones", "Search it too", "Not the library");
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_6 = searcher_6; let may_6 = g.may_search(searcher_6, owner_6); let rule_count_6 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_6: Vec<ObjectId> = (if may_6 { cantrip_mechanics::library::searched_cards(g, owner_6) } else { Vec::new() }).into_iter().chain(g.state().zone(Zone::graveyard(owner_6)).iter().copied()).chain(g.state().zone(Zone::hand(owner_6)).iter().copied()).collect::<Vec<ObjectId>>(); let can_6: Vec<ObjectId> = { let q: &dyn Query = g; shown_6.iter().copied().filter(|&id| q.chars(id).name == "Altanak, the Thrice-Called").collect() };
let zones_6: Vec<ObjectId> = g.state().zone(Zone::graveyard(owner_6)).iter().chain(g.state().zone(Zone::hand(owner_6)).iter()).copied().collect(); let searched_6 = cantrip_mechanics::library::ask_may_search(g, searcher_6, asker.source, &ASK_6)?; let found_6: Vec<ObjectId> = if searched_6 { { let rule = cantrip_mechanics::select::Selection::listed(rule_count_6, &shown_6, can_6.clone()); let found = if shown_6.is_empty() { Vec::new() } else { match g.ask(searcher_6, RequestKind::SearchLibrary { source: asker.source, library: shown_6.clone(), rule: Box::new(rule), destination: SearchDestination::Battlefield })? { Answer::Cards(cards) => cards, _ => Vec::new() } }; if may_6 { g.searched(searcher_6, owner_6); } found } } else { let rule = { let in_zones: Vec<ObjectId> = can_6.iter().copied().filter(|o| zones_6.contains(o)).collect(); cantrip_mechanics::select::Selection::listed(rule_count_6, &zones_6, in_zones) }; let any = cantrip_api::select::first_answer(&*g, &rule, searcher_6, asker.source, &zones_6, true).is_some_and(|taken| !taken.is_empty()); if any { cantrip_mechanics::choose::select(g, searcher_6, asker.source, Purpose::PutOntoBattlefield, zones_6.clone(), rule)? } else { Vec::new() } };
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; found_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
if { let q: &dyn Query = g; searched_6 } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_9(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && q.chars(id).name == "Say Its Name") }

pub static CARD: CardDef = CardDef { name: "Say Its Name", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(EXILE_THIS.and(planned_value!("engine: no cost moves these objects", Cost::NONE => Cost { moves: Some(MovesObjects { from: CostZone::Graveyard(Whose::Yours), to: CostDestination::Exile, count: CostCount::Exactly(2), filter: ability_1_9 }), ..Cost::NONE }))), ability_1, "Exile this card and two other cards named Say Its Name from your graveyard: Search your graveyard, hand, and/or library for a card named Altanak, the Thrice-Called and put it onto the battlefield. If you search your library this way, shuffle. Activate only as a sorcery.").targets(&[]).sorcery_speed().from(ZoneKind::Graveyard)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
