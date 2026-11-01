// Restore Balance
// status: planned
// requires: per-player DiscardAllBut
// requires: per-player SacrificeAllBut
// Suspend 6—{W}
// Each player chooses a number of lands they control equal to the number of lands controlled by the player who controls the fewest, then sacrifices the rest. Players sacrifice creatures and discard cards the same way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-sacrifice-all-but" => g.begin_batch(););
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let keep_1 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == p && q.chars(id).is(CardType::Land)).count() as i64).min().unwrap_or(0)).max(0) as u32 }; let mut picks_1: Vec<(PlayerId, ObjectId)> = Vec::new(); for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { let own: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.controller(o) == player).collect(); let kept = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Keep, own.clone(), cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::AsManyAsCan(keep_1), &[]))?; for o in own { if !kept.contains(&o) && g.can_sacrifice(player, o) { picks_1.push((player, o)); } } } let sacrificed_1 = g.sacrifice_each(&picks_1);
 each_2[index_0] = sacrificed_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_3[index_0] = { let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_3.extend(each_3[index_0].iter().copied());
   } 
planned!("per-player-sacrifice-all-but" => g.end_batch(););
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-sacrifice-all-but" => g.begin_batch(););
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let keep_5 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == p && q.chars(id).is(CardType::Creature)).count() as i64).min().unwrap_or(0)).max(0) as u32 }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new(); for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { let own: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.controller(o) == player).collect(); let kept = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Keep, own.clone(), cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::AsManyAsCan(keep_5), &[]))?; for o in own { if !kept.contains(&o) && g.can_sacrifice(player, o) { picks_5.push((player, o)); } } } let sacrificed_5 = g.sacrifice_each(&picks_5);
 each_6[index_4] = sacrificed_5.clone(); all_6.extend(each_6[index_4].iter().copied());
each_7[index_4] = { let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_7.extend(each_7[index_4].iter().copied());
   } 
planned!("per-player-sacrifice-all-but" => g.end_batch(););
g.end_sentence();
let mut players_8: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-discard-all-but" => g.begin_batch(););
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_8.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_8.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { let keep_9 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| q.state().zone(Zone::hand(p)).len() as i64).min().unwrap_or(0)).max(0) as u32 }; let mut cards_9: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { let hand = g.state().zone(Zone::hand(player)).to_vec(); let extra = (hand.len() as u32).saturating_sub(keep_9); cards_9.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, extra)?); } let discarded_9 = g.discard_together(&cards_9);
 each_10[index_8] = discarded_9.clone(); all_10.extend(each_10[index_8].iter().copied());
each_11[index_8] = { let q: &dyn Query = g; discarded_9.clone() }; all_11.extend(each_11[index_8].iter().copied());
   } 
planned!("per-player-discard-all-but" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Restore Balance", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[cantrip_mechanics::suspend::ability(ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Suspend")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, suspend: Some(SuspendDef { count: 6, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] } }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
