// Variable Solutions
// status: planned
// requires: operation Conjure
// If X is 1, seek a basic land card and put it onto the battlefield tapped.
// If X is 2, each opponent sacrifices an artifact of their choice.
// If X is 3, conjure a random creature card with mana value X onto the battlefield.
// If X is 4 or more, you gain 2 life. Then do all of the above.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_28: Vec<ObjectId> = Vec::new();
let mut hoisted_27: Vec<ObjectId> = Vec::new();
let mut hoisted_26: Vec<ObjectId> = Vec::new();
let mut hoisted_25: Vec<ObjectId> = Vec::new();
let mut hoisted_24: Vec<ObjectId> = Vec::new();
let mut hoisted_23: Vec<ObjectId> = Vec::new();
let mut hoisted_22: Vec<ObjectId> = Vec::new();
let mut hoisted_21: Vec<ObjectId> = Vec::new();
let mut hoisted_20: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (1i64) } {
let mut pool_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_0: Vec<ObjectId> = Vec::new(); while picked_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); picked_0.push(pool_0.remove(at)); }
let objects_1 = { let q: &dyn Query = g; picked_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_2 = ({ let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_2 = { let rows_2: Vec<(ObjectId, PlayerId, bool)> = objects_2.iter().map(|&o| (o, player_2, true)).collect(); let came_2 = g.put_all_onto_battlefield_as(&rows_2)?; came_2.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_3.extend(moved_1.clone());
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(picked_0.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_7.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (2i64) } {
let objects_8 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_8: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_8 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_8 { let eligible: Vec<ObjectId> = objects_8.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_8.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_8.clone(), &[]))? }; for object in taken { picks_8.push((player, object)); } }
let sacrificed_8 = g.sacrifice_each(&picks_8);
g.end_sentence();
hoisted_9.extend(sacrificed_8.clone());
hoisted_10.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (3i64) } {
let count_11: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_11: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_11.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::AtRandom(CardsInOracle::And(CardsInOracle::IsCardtype(CardType::Creature), CardsInOracle::ManaValueIs("Comparison(Equal, AnnouncedX)"))), count_11, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_12.extend(conjured_11.clone());
hoisted_13.extend(conjured_11.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) >= (4i64) } {
let amount_14 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_14 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_14 { g.gain_life(player, amount_14); }
g.end_sentence();
let mut pool_15 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let count_15 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_15: Vec<ObjectId> = Vec::new(); while picked_15.len() < count_15 && !pool_15.is_empty() { let at = g.choose_at_random(pool_15.len()); picked_15.push(pool_15.remove(at)); }
let objects_16 = { let q: &dyn Query = g; picked_15.clone() }; let moved_16 = g.return_to_hand(&objects_16);
g.end_sentence();
let player_17 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_17 = ({ let q: &dyn Query = g; (moved_16.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_17 = { let rows_17: Vec<(ObjectId, PlayerId, bool)> = objects_17.iter().map(|&o| (o, player_17, true)).collect(); let came_17 = g.put_all_onto_battlefield_as(&rows_17)?; came_17.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let objects_18 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; let players_18 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_18: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_18 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_18 { let eligible: Vec<ObjectId> = objects_18.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_18.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_18.clone(), &[]))? }; for object in taken { picks_18.push((player, object)); } }
let sacrificed_18 = g.sacrifice_each(&picks_18);
g.end_sentence();
let count_19: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_19: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_19.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::AtRandom(CardsInOracle::And(CardsInOracle::IsCardtype(CardType::Creature), CardsInOracle::ManaValueIs("Comparison(Equal, AnnouncedX)"))), count_19, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
hoisted_20.extend(moved_16.clone());
hoisted_21.extend(moved_17.clone());
hoisted_22.extend(sacrificed_18.clone());
hoisted_23.extend(conjured_19.clone());
hoisted_24.extend(picked_15.clone());
hoisted_25.extend(conjured_19.clone());
hoisted_26.extend({ let q: &dyn Query = g; objects_16.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_27.extend({ let q: &dyn Query = g; objects_17.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_28.extend({ let q: &dyn Query = g; objects_18.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Variable Solutions", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
