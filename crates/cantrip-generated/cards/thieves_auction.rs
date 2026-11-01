// Thieves' Auction
// status: planned
// requires: per-player move under each player's control
// Exile all nontoken permanents. Starting with you, each player chooses one of the exiled cards and puts it onto the battlefield tapped under their control. Repeat this process until all cards exiled this way have been chosen.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| !(q.state().obj(id).token)).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
loop { let mut again_1 = false;
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_2 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_2.iter().position(|&p| Some(p) == first_2) { players_2.rotate_left(at); }
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let selected_3_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_0.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutOntoBattlefield, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
 each_4[index_2] = selected_3.clone(); all_4.extend(each_4[index_2].iter().copied());
   } 
g.end_sentence();
planned!("per-player-move-under-each-player-s-control" => g.begin_batch(););
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let player_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let objects_5 = ({ let q: &dyn Query = g; each_4[index_2].clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_5 = { let rows_5: Vec<(ObjectId, PlayerId, bool)> = objects_5.iter().map(|&o| (o, player_5, true)).collect(); let came_5 = g.put_all_onto_battlefield_as(&rows_5)?; came_5.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
 each_6[index_2] = moved_5.clone(); all_6.extend(each_6[index_2].iter().copied());
each_7[index_2] = { let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_7.extend(each_7[index_2].iter().copied());
   } 
planned!("per-player-move-under-each-player-s-control" => g.end_batch(););
g.end_sentence();
if { let q: &dyn Query = g; ((((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_0.clone()).contains(&id)).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
again_1 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_9.extend(all_6.clone());
hoisted_10.extend(all_4.clone());
hoisted_11.extend({ let q: &dyn Query = g; all_7.clone() });
if !again_1 { break; } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Thieves' Auction", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 3, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
