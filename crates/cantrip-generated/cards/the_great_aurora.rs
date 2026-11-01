// The Great Aurora
// status: planned
// requires: numeric expression Earlier
// requires: per-player ChangeZone
// Each player shuffles all cards from their hand and all permanents they own into their library, then draws that many cards. Each player may put any number of land cards from their hand onto the battlefield. Exile The Great Aurora.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-change-zone" => g.begin_batch(););
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_2 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in q.state().battlefield().iter().copied().filter(|&id| Some(bound_0) == Some(q.state().obj(id).owner)).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_2 = g.put_into_library(&objects_2, LibraryPlace::ShuffledIn);
 each_3[index_0] = moved_2.clone(); all_3.extend(each_3[index_0].iter().copied());
each_4[index_0] = { let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_4.extend(each_4[index_0].iter().copied());
   } 
planned!("per-player-change-zone" => g.end_batch(););
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { let amount_6 = { let q: &dyn Query = g; (planned_value!("numeric expression Earlier", 0i64 => q.earlier_objects(item, EarlierObjects::Shuffled, Some(bound_0)).into_iter().count() as i64)).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
    } 
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_7.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { let selected_8_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::PutOntoBattlefield, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
 each_9[index_7] = selected_8.clone(); all_9.extend(each_9[index_7].iter().copied());
   } 
g.end_sentence();
let objects_10 = ({ let q: &dyn Query = g; all_9.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_10 = { let rows_10: Vec<(ObjectId, PlayerId, bool)> = objects_10.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_10 = g.put_all_onto_battlefield_as(&rows_10)?; came_10.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let objects_11 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_11 = g.exile_all(&objects_11);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Great Aurora", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
