// Vision Quest
// status: ready
// Search your library and/or graveyard for an artifact creature card with mana value X or less and put it onto the battlefield with X additional +1/+1 counters on it. If X is 4 or greater, it gains haste until end of turn. If you search your library this way, shuffle.
#[allow(unused_imports)] use cantrip_api::*;
static ASK_0: Question = Question::new("SearchLibrary", "search your library as well as your graveyard?", "Then shuffle. If you don't, you search only your graveyard", "Search both", "Graveyard only");
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let zone_0 = Zone::graveyard(searcher_0); let mut shown_0 = cantrip_mechanics::library::searched_cards(g, searcher_0); shown_0.extend(g.state().zone(zone_0).iter().copied()); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (i64::from(item.x)))).collect() }; let (found_0, searched_0) = cantrip_mechanics::library::search_zone_and_or_library(g, searcher_0, asker.source, zone_0, SearchDestination::Battlefield, Purpose::PutOntoBattlefield, &ASK_0, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; found_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let counters = [(CounterKind::PlusOne, { let q: &dyn Query = g; (i64::from(item.x)).min(i16::MAX.into()) as i16 })]; g.put_all_onto_battlefield_with_counters(&rows_1, &counters)? };
if { let q: &dyn Query = g; searched_0 } {
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) >= (4i64) } {
let objects_3 = { let q: &dyn Query = g; ((moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
hoisted_4.extend(objects_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vision Quest", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
