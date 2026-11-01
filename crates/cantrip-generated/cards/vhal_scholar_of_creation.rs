// Vhal, Scholar of Creation
// status: planned
// requires: counter kind Study
// requires: engine: no specialize event
// When this creature specializes, remove all study counters from it. Seek two creature cards with mana value less than or equal to the number of study counters removed this way. Put one of them onto the battlefield and shuffle the other into your library.
// {T}: Draw a card, then discard a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_0: i64 = 0;
let objects_0_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_0_0 = i16::MAX; cantrip_api::instruction(g, |g| { for object in objects_0_0 { removed_0 += i64::from(g.remove_counters(object, planned_value!("counter kind Study", CounterKind::Charge => CounterKind::Study), amount_0_0)); } });
g.end_sentence();
let mut pool_1 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= ((removed_0 as i64)))).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let mut picked_1: Vec<ObjectId> = Vec::new(); while picked_1.len() < count_1 && !pool_1.is_empty() { let at = g.choose_at_random(pool_1.len()); picked_1.push(pool_1.remove(at)); }
let objects_2 = { let q: &dyn Query = g; picked_1.clone() }; let moved_2 = g.return_to_hand(&objects_2);
g.end_sentence();
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_2.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutOntoBattlefield, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; selected_3.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, player_4, false)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_2.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let moved_5 = g.put_into_library(&objects_5, LibraryPlace::ShuffledIn);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
let amount_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_7 = Vec::new(); for player in players_7 { cards_7.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_7)?); } let discarded_7 = g.discard_together(&cards_7);
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-specialize-event" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Specialized { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Vhal, Scholar of Creation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no specialize event", EventMask(0) => EventMask(EventMask::SPECIALIZED.0)), ability_0_event), ability_0, "When this creature specializes, remove all study counters from it. Seek two creature cards with mana value less than or equal to the number of study counters removed this way. Put one of them onto the battlefield and shuffle the other into your library.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: Draw a card, then discard a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
