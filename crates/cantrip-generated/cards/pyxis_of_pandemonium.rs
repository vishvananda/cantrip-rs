// Pyxis of Pandemonium
// status: planned
// requires: earlier objects It
// requires: per-player KeywordEach
// requires: per-player move under each player's control
// {T}: Each player exiles the top card of their library face down.
// {7}, {T}, Sacrifice this artifact: Each player turns face up all cards they own exiled with this artifact, then puts all permanent cards among them onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend({ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() });
    } 
let objects_3 = { let q: &dyn Query = g; gathered_1.clone() }; let by_3 = Some(g.linked(item)); let moved_3 = g.exile_face_down(&objects_3, by_3);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-keyword-each" => g.begin_batch(););
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { for bound_4294967290 in { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (Some(bound_0) == Some(q.state().obj(id).owner) && q.exiled_with(q.linked_to(asker)).contains(&id))).collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967290).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.turn_face_up(object, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }); }
}
    } 
planned!("per-player-keyword-each" => g.end_batch(););
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-move-under-each-player-s-control" => g.begin_batch(););
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_7.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_7.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { let player_8 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let objects_8 = ({ let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && (planned_value!("earlier objects It", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::It))).contains(&id))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_8 = { let rows_8: Vec<(ObjectId, PlayerId, bool)> = objects_8.iter().map(|&o| (o, player_8, false)).collect(); let came_8 = g.put_all_onto_battlefield_as(&rows_8)?; came_8.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
 each_9[index_7] = moved_8.clone(); all_9.extend(each_9[index_7].iter().copied());
each_10[index_7] = { let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_10.extend(each_10[index_7].iter().copied());
   } 
planned!("per-player-move-under-each-player-s-control" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Pyxis of Pandemonium", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{T}: Each player exiles the top card of their library face down.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{7}, {T}, Sacrifice this artifact: Each player turns face up all cards they own exiled with this artifact, then puts all permanent cards among them onto the battlefield.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
