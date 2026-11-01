// Natural Balance
// status: planned
// requires: per-player SacrificeAllBut
// Each player who controls six or more lands chooses five lands they control and sacrifices the rest. Each player who controls four or fewer lands may search their library for up to X basic land cards and put them onto the battlefield, where X is five minus the number of lands they control. Then each player who searched their library this way shuffles.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (6i64)).collect::<Vec<PlayerId>>() };
planned!("per-player-sacrifice-all-but" => g.begin_batch(););
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let keep_1 = { let q: &dyn Query = g; (5i64).max(0) as u32 }; let mut picks_1: Vec<(PlayerId, ObjectId)> = Vec::new(); for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { let own: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.controller(o) == player).collect(); let kept = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Keep, own.clone(), cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::AsManyAsCan(keep_1), &[]))?; for o in own { if !kept.contains(&o) && g.can_sacrifice(player, o) { picks_1.push((player, o)); } } } let sacrificed_1 = g.sacrifice_each(&picks_1);
 each_2[index_0] = sacrificed_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_3[index_0] = { let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_3.extend(each_3[index_0].iter().copied());
   } 
planned!("per-player-sacrifice-all-but" => g.end_batch(););
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) <= (4i64)).collect::<Vec<PlayerId>>() };
let mut guard_4_0 = vec![false; players_4.len()];
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_4_0[index_4] = true;
let searcher_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let owner_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo(((5i64 - (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64)).max(0) as u32) }; let shown_5 = cantrip_mechanics::library::searched_cards(g, owner_5); let can_5: Vec<ObjectId> = { let q: &dyn Query = g; shown_5.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_5 = cantrip_mechanics::library::search(g, searcher_5, owner_5, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(rule_count_5, &shown_5, can_5))?;
 each_6[index_4] = found_5.clone(); all_6.extend(each_6[index_4].iter().copied());
  }
 } 
g.end_sentence();
let objects_7 = ({ let q: &dyn Query = g; all_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { if guard_4_0[index_4] {
for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
   }
 } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Natural Balance", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
