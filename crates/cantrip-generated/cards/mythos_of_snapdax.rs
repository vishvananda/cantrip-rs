// Mythos of Snapdax
// status: ready
// Each player chooses an artifact, a creature, an enchantment, and a planeswalker from among the nonland permanents they control, then sacrifices the rest. If {B}{R} was spent to cast this spell, you choose the permanents for each player instead.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| ((Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::B].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (1i64) && (Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::R].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (1i64))) } {
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut picked_1: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_1.iter().copied().filter(|id| !picked_1.contains(id)).filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_1, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_1.contains(&o) { picked_1.push(o); } } }
 each_2[index_0] = picked_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!((all_2.clone()).contains(&id)) && !(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
gathered_4.extend(picks_5.iter().copied());
    } 
let sacrificed_4 = g.sacrifice_each(&gathered_4);
g.end_sentence();
hoisted_6.extend(sacrificed_4.clone());
hoisted_7.extend(all_2.clone());
hoisted_8.extend({ let q: &dyn Query = g; gathered_4.iter().map(|pick| pick.1).collect::<Vec<ObjectId>>().iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_9.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let player_10 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_10 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let mut picked_10: Vec<ObjectId> = Vec::new();
{ let open = { let q: &dyn Query = g; pool_10.iter().copied().filter(|id| !picked_10.contains(id)).filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_10, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_10.contains(&o) { picked_10.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_10.iter().copied().filter(|id| !picked_10.contains(id)).filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_10, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_10.contains(&o) { picked_10.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_10.iter().copied().filter(|id| !picked_10.contains(id)).filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_10, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_10.contains(&o) { picked_10.push(o); } } }
{ let open = { let q: &dyn Query = g; pool_10.iter().copied().filter(|id| !picked_10.contains(id)).filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>() }; let one = cantrip_mechanics::choose::cards(g, player_10, asker.source, Purpose::ChooseAsEnters, open, 1, false)?; for o in one { if !picked_10.contains(&o) { picked_10.push(o); } } }
 each_11[index_9] = picked_10.clone(); all_11.extend(each_11[index_9].iter().copied());
   } 
g.end_sentence();
let mut gathered_12: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let objects_13 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!((each_11[index_9].clone()).contains(&id)) && !(q.chars(id).is(CardType::Land)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let players_13 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_13: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_13 { let controller = g.controller(object); if players_13.contains(&controller) && g.can_sacrifice(controller, object) { picks_13.push((controller, object)); } }
gathered_12.extend(picks_13.iter().copied());
    } 
let sacrificed_12 = g.sacrifice_each(&gathered_12);
g.end_sentence();
hoisted_14.extend(sacrificed_12.clone());
hoisted_15.extend(all_11.clone());
hoisted_16.extend({ let q: &dyn Query = g; gathered_12.iter().map(|pick| pick.1).collect::<Vec<ObjectId>>().iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mythos of Snapdax", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
