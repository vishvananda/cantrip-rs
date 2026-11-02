// The Great Aurora
// status: ready
// Each player shuffles all cards from their hand and all permanents they own into their library, then draws that many cards. Each player may put any number of land cards from their hand onto the battlefield. Exile The Great Aurora.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in q.state().battlefield().iter().copied().filter(|&id| Some(bound_0) == Some(q.state().obj(id).owner)).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } });
    } 
let objects_3 = { let q: &dyn Query = g; gathered_1.clone() }; let moved_3 = g.put_into_library(&objects_3, LibraryPlace::ShuffledIn);
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let amount_5 = { let q: &dyn Query = g; (((moved_3.clone()).into_iter().filter(|&id: &ObjectId| q.state().obj(id).owner == bound_0).collect::<Vec<ObjectId>>()).into_iter().count() as i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
    } 
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_6.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { let selected_7_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_7_candidates = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let selected_7_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let selected_7 = { cantrip_mechanics::choose::select(g, selected_7_player, asker.source, Purpose::PutOntoBattlefield, selected_7_candidates, cantrip_mechanics::select::Selection::of(selected_7_count, &[]))? };
 each_8[index_6] = selected_7.clone(); all_8.extend(each_8[index_6].iter().copied());
   } 
g.end_sentence();
let objects_9 = ({ let q: &dyn Query = g; all_8.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_9 = { let rows_9: Vec<(ObjectId, PlayerId, bool)> = objects_9.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_9 = g.put_all_onto_battlefield_as(&rows_9)?; came_9.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let objects_10 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_10 = g.exile_all(&objects_10);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Great Aurora", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
