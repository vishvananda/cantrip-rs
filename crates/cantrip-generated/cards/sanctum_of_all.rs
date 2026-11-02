// Sanctum of All
// status: ready
// At the beginning of your upkeep, you may search your library and/or graveyard for a Shrine card and put it onto the battlefield. If you search your library this way, shuffle.
// If a triggered ability of another Shrine you control triggers while you control six or more Shrines, that ability triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
static ASK_1: Question = Question::new("SearchLibrary", "search your library as well as your graveyard?", "Then shuffle. If you don't, you search only your graveyard", "Search both", "Graveyard only");
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let zone_1 = Zone::graveyard(searcher_1); let mut shown_1 = cantrip_mechanics::library::searched_cards(g, searcher_1); shown_1.extend(g.state().zone(zone_1).iter().copied()); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Shrine))).collect() }; let (found_1, searched_1) = cantrip_mechanics::library::search_zone_and_or_library(g, searcher_1, asker.source, zone_1, SearchDestination::Battlefield, Purpose::PutOntoBattlefield, &ASK_1, cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1))?;
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
#[allow(unused_variables)] fn ability_1_0_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Shrine)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (6i64) && (triggering.permanent && (!(Some(asker.source) == Some(triggering.source)) && q.chars(triggering.source).has_subtype(q.words(asker.source).subtype(Subtype::Shrine)) && Some(asker.controller) == Some(triggering.controller))) && true }

pub static CARD: CardDef = CardDef { name: "Sanctum of All", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Shrine]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, you may search your library and/or graveyard for a Shrine card and put it onto the battlefield. If you search your library this way, shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_1_0_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
