// Living Death
// status: ready
// Each player exiles all creature cards from their graveyard, then sacrifices all creatures they control, then puts all cards they exiled this way onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_0) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() });
    } 
let objects_3 = { let q: &dyn Query = g; gathered_1.clone() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_4 { let controller = g.controller(object); if players_4.contains(&controller) && g.can_sacrifice(controller, object) { picks_4.push((controller, object)); } }
let sacrificed_4 = g.sacrifice_each(&picks_4);
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_6: Vec<ObjectId> = Vec::new();
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { gathered_6.extend({ let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((moved_3.clone()).into_iter().filter(|&o| Some(q.state().obj(o).owner) == Some(bound_0)).collect::<Vec<ObjectId>>()).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() });
    } 
let objects_8 = ({ let q: &dyn Query = g; gathered_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_8 = { let rows_8: Vec<(ObjectId, PlayerId, bool)> = objects_8.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_8 = g.put_all_onto_battlefield_as(&rows_8)?; came_8.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Living Death", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
