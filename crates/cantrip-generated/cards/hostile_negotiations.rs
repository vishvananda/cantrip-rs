// Hostile Negotiations
// status: planned
// requires: keyword action TurnPileFaceUp
// requires: operation ChoosePile
// requires: operation Move
// Exile the top three cards of your library in a face-down pile, then exile the top three cards of your library in another face-down pile. Look at the cards in each pile, then turn a pile of your choice face up. An opponent chooses one of those piles. Put that pile into your hand and the other into your graveyard. You lose 3 life.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((3i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0: Vec<ObjectId> = planned_value!("operation Move", Vec::<ObjectId>::new() => cantrip_mechanics::enter::in_state(g, asker, &objects_0, Destination::Exile, Arrival { tapped: false, counters: &[], states: &[ArrivalState::ExilePile { shuffled: false }] }, true)?);
g.end_sentence();
let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((3i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_1: Vec<ObjectId> = planned_value!("operation Move", Vec::<ObjectId>::new() => cantrip_mechanics::enter::in_state(g, asker, &objects_1, Destination::Exile, Arrival { tapped: false, counters: &[], states: &[ArrivalState::ExilePile { shuffled: false }] }, true)?);
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_2 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_1.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; g.look_at(player_2, &objects_2);
g.end_sentence();
planned!("keyword-action-turn-pile-face-up" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::piles::turn_pile_face_up(g, player, asker.source)?; });
g.end_sentence();
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if among.len() > 1 { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
planned!("operation-choose-pile" => cantrip_mechanics::choose::of_piles(g, { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }, asker.source, None, false, &[], None, None)?);

g.end_sentence();
let objects_7 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_4)).contains(&id)).collect::<Vec<ObjectId>>() }; let moved_7 = g.return_to_hand(&objects_7);
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_7.clone()).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_8 = { let moves: Vec<(ObjectId, Zone)> = objects_8.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
let amount_9 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.lose_life(player, amount_9); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Hostile Negotiations", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
