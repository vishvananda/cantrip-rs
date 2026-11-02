// Agency Outfitter
// status: ready
// Flying
// When this creature enters, you may search your graveyard, hand and/or library for a card named Magnifying Glass and/or a card named Thinking Cap and put them onto the battlefield. If you search your library this way, shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn clauses_1(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let asker = &Asker::new(choice.source, q.controller(choice.source)); clauses_verdict(choice, &mut [((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).name == "Magnifying Glass"), ((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).name == "Thinking Cap")]) }
static ASK_1: Question = Question::new("SearchLibrary", "search your library as well?", "Then shuffle. If you don't, you search only the other zones", "Search it too", "Not the library");
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = searcher_1; let may_1 = g.may_search(searcher_1, owner_1); let shown_1: Vec<ObjectId> = (if may_1 { cantrip_mechanics::library::searched_cards(g, owner_1) } else { Vec::new() }).into_iter().chain(g.state().zone(Zone::graveyard(owner_1)).iter().copied()).chain(g.state().zone(Zone::hand(owner_1)).iter().copied()).collect::<Vec<ObjectId>>();
let zones_1: Vec<ObjectId> = g.state().zone(Zone::graveyard(owner_1)).iter().chain(g.state().zone(Zone::hand(owner_1)).iter()).copied().collect(); let searched_1 = cantrip_mechanics::library::ask_may_search(g, searcher_1, asker.source, &ASK_1)?; let found_1: Vec<ObjectId> = if searched_1 { { let rule = cantrip_mechanics::select::Selection::new(cantrip_mechanics::select::up_to((1i64).max(0) as u32 + (1i64).max(0) as u32)).with(cantrip_api::select::part(&(clauses_1 as SelectFn), 0, "one for each thing searched for")); let found = if shown_1.is_empty() { Vec::new() } else { match g.ask(searcher_1, RequestKind::SearchLibrary { source: asker.source, library: shown_1.clone(), rule: Box::new(rule), destination: SearchDestination::Battlefield })? { Answer::Cards(cards) => cards, _ => Vec::new() } }; if may_1 { g.searched(searcher_1, owner_1); } found } } else { let rule = cantrip_mechanics::select::Selection::new(cantrip_mechanics::select::up_to((1i64).max(0) as u32 + (1i64).max(0) as u32)).with(cantrip_api::select::part(&(clauses_1 as SelectFn), 0, "one for each thing searched for")); let any = cantrip_api::select::first_answer(&*g, &rule, searcher_1, asker.source, &zones_1, true).is_some_and(|taken| !taken.is_empty()); if any { cantrip_mechanics::choose::select(g, searcher_1, asker.source, Purpose::PutOntoBattlefield, zones_1.clone(), rule)? } else { Vec::new() } };
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; found_1.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows_2: Vec<(ObjectId, PlayerId, bool)> = objects_2.iter().map(|&o| (o, player_2, false)).collect(); let came_2 = g.put_all_onto_battlefield_as(&rows_2)?; came_2.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
if { let q: &dyn Query = g; searched_1 } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
}
g.end_sentence();
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(found_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn clauses_fit(cards: &[ObjectId], clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> bool { let Some((&card, rest)) = cards.split_first() else { return true; }; for i in 0..clauses.len() { if clauses[i].0 > 0 && (clauses[i].1)(card) { clauses[i].0 -= 1; let fits = clauses_fit(rest, clauses); clauses[i].0 += 1; if fits { return true; } } } false }
fn clauses_verdict(choice: &Choice, clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> Verdict { let complete = clauses_fit(choice.selected, clauses); let addable = Bits::of(choice.candidates.len(), |i| { let card = choice.candidates[i]; !choice.selected.contains(&card) && { let with: Vec<ObjectId> = choice.selected.iter().copied().chain([card]).collect(); clauses_fit(&with, clauses) } }); Verdict { complete, addable } }

pub static CARD: CardDef = CardDef { name: "Agency Outfitter", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx,Subtype::Detective]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, you may search your graveyard, hand and/or library for a card named Magnifying Glass and/or a card named Thinking Cap and put them onto the battlefield. If you search your library this way, shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
