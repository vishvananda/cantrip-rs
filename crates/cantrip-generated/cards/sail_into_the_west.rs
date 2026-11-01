// Sail into the West
// status: planned
// requires: condition MostVotes
// requires: operation Vote
// Will of the council — Starting with you, each player votes for return or embark. If return gets more votes, each player returns up to two cards from their graveyard to their hand, then you exile Sail into the West. If embark gets more votes or the vote is tied, each player may discard their hand and draw seven cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let voters_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_1, cantrip_mechanics::vote::Ballot::Words(&["Return", "Embark"]), false)?);
    } 
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition MostVotes", false => g.votes().most(option)) } {
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let selected_3_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((2i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutIntoHand, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
 each_4[index_2] = selected_3.clone(); all_4.extend(each_4[index_2].iter().copied());
   } 
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; all_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_6 = g.exile_all(&objects_6);
g.end_sentence();
hoisted_7.extend(moved_5.clone());
hoisted_8.extend(moved_6.clone());
hoisted_9.extend(all_4.clone());
hoisted_10.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_11.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition MostVotes", false => g.votes().most(option)) } {
let mut players_12: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_12_0 = vec![false; players_12.len()];
let mut gathered_13: Vec<ObjectId> = Vec::new();
 for (index_12, bound_0) in players_12.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_12_0[index_12] = true;
let amount_14 = { let q: &dyn Query = g; ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; gathered_13.extend(cantrip_mechanics::choose_from_hand(g, bound_0, asker.source, amount_14)?);
   }
 } 
let cards_15 = { let q: &dyn Query = g; gathered_13.clone() }; let discarded_15 = g.discard_together(&cards_15);
g.end_sentence();
 for (index_12, bound_0) in players_12.iter().copied().enumerate() { if guard_12_0[index_12] {
let amount_16 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_16 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_16 { g.draw(player, amount_16); }
   }
 } 
g.end_sentence();
hoisted_17.extend(discarded_15.clone());
hoisted_18.extend({ let q: &dyn Query = g; discarded_15.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sail into the West", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
