// Enigma Ridges
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: subtype Echoir
// requires: trigger event Player ChaosEnsues
// requires: trigger event Player Planeswalks
// When you planeswalk to Enigma Ridges, each player who controls fewer lands than the player who controls the most lands searches their library for a number of basic land cards less than or equal to the difference, reveals them, puts them into their hand, then shuffles.
// Whenever chaos ensues, draw a card, then you may put a land card from your hand onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let value_0: i64 = { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == p && q.chars(id).is(CardType::Land)).count() as i64).max().unwrap_or(0) };
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) < (value_0)).collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let searcher_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let owner_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).saturating_sub(value_0)).clamp(0, 4294967295) as u32) }; let shown_2 = cantrip_mechanics::library::searched_cards(g, owner_2); let can_2: Vec<ObjectId> = { let q: &dyn Query = g; shown_2.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_2 = cantrip_mechanics::library::search(g, searcher_2, owner_2, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_2, &shown_2, can_2))?;
 each_3[index_1] = found_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; all_3.clone() }; let mut owners_4: Vec<PlayerId> = objects_4.iter().map(|o| g.state().obj(*o).owner).collect(); owners_4.dedup(); for owner in owners_4 { let cards: Vec<ObjectId> = objects_4.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; all_3.clone() }; let moved_5 = g.return_to_hand_revealed(&objects_5);
g.end_sentence();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.draw(player, amount_7); }
g.end_sentence();
{
let selected_8_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::PutOntoBattlefield, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
if !selected_8.is_empty() { 
let player_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_9 = ({ let q: &dyn Query = g; selected_8.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_9 = { let rows_9: Vec<(ObjectId, PlayerId, bool)> = objects_9.iter().map(|&o| (o, player_9, false)).collect(); let came_9 = g.put_all_onto_battlefield_as(&rows_9)?; came_9.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend(selected_8.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-planeswalks" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Planeswalked { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Enigma Ridges", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Echoir", Subtype::Plains => Subtype::Echoir)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player Planeswalks", EventMask(0) => EventMask(EventMask::PLANESWALKED.0)), ability_0_event), ability_0, "When you planeswalk to Enigma Ridges, each player who controls fewer lands than the player who controls the most lands searches their library for a number of basic land cards less than or equal to the difference, reveals them, puts them into their hand, then shuffles.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, draw a card, then you may put a land card from your hand onto the battlefield.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
