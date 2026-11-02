// Mishra, Artificer Prodigy
// status: ready
// Whenever you cast an artifact spell, you may search your graveyard, hand, and/or library for a card with the same name as that spell and put it onto the battlefield. If you search your library this way, shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).is(CardType::Artifact) { return Some(Fired { subject, data }); } }
 None }
static ASK_1: Question = Question::new("SearchLibrary", "search your library as well?", "Then shuffle. If you don't, you search only the other zones", "Search it too", "Not the library");
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = searcher_1; let may_1 = g.may_search(searcher_1, owner_1); let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_1: Vec<ObjectId> = (if may_1 { cantrip_mechanics::library::searched_cards(g, owner_1) } else { Vec::new() }).into_iter().chain(g.state().zone(Zone::graveyard(owner_1)).iter().copied()).chain(g.state().zone(Zone::hand(owner_1)).iter().copied()).collect::<Vec<ObjectId>>(); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| item.subject.object().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() };
let zones_1: Vec<ObjectId> = g.state().zone(Zone::graveyard(owner_1)).iter().chain(g.state().zone(Zone::hand(owner_1)).iter()).copied().collect(); let searched_1 = cantrip_mechanics::library::ask_may_search(g, searcher_1, asker.source, &ASK_1)?; let found_1: Vec<ObjectId> = if searched_1 { { let rule = cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1.clone()); let found = if shown_1.is_empty() { Vec::new() } else { match g.ask(searcher_1, RequestKind::SearchLibrary { source: asker.source, library: shown_1.clone(), rule: Box::new(rule), destination: SearchDestination::Battlefield })? { Answer::Cards(cards) => cards, _ => Vec::new() } }; if may_1 { g.searched(searcher_1, owner_1); } found } } else { let rule = { let in_zones: Vec<ObjectId> = can_1.iter().copied().filter(|o| zones_1.contains(o)).collect(); cantrip_mechanics::select::Selection::listed(rule_count_1, &zones_1, in_zones) }; let any = cantrip_api::select::first_answer(&*g, &rule, searcher_1, asker.source, &zones_1, true).is_some_and(|taken| !taken.is_empty()); if any { cantrip_mechanics::choose::select(g, searcher_1, asker.source, Purpose::PutOntoBattlefield, zones_1.clone(), rule)? } else { Vec::new() } };
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

pub static CARD: CardDef = CardDef { name: "Mishra, Artificer Prodigy", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast an artifact spell, you may search your graveyard, hand, and/or library for a card with the same name as that spell and put it onto the battlefield. If you search your library this way, shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
