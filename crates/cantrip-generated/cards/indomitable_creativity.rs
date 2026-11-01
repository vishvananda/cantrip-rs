// Indomitable Creativity
// status: planned
// requires: object collection Earlier
// requires: object collection MovedBy
// requires: per-player move under each player's control
// Destroy X target artifacts and/or creatures. For each permanent destroyed this way, its controller reveals cards from the top of their library until an artifact or creature card is revealed and exiles that card. Those players put the exiled cards onto the battlefield, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).counted()];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() } {
let owner_1 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_1, rest_1) = cantrip_mechanics::library::reveal_until_n(g, owner_1, count_1, &|q: &dyn Query, id: ObjectId| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature))); let shown_1 = found_1.len() + rest_1.len(); let objects_1: Vec<ObjectId> = g.state().zone(Zone::library(owner_1)).iter().copied().take(shown_1).collect();
let objects_2 = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
hoisted_3.extend(moved_2.clone());
hoisted_4.extend(objects_1.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| ((planned_value!("object collection MovedBy", Vec::<ObjectId>::new() => q.moved_by(result_2, Some(bound_0)))).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
planned!("per-player-move-under-each-player-s-control" => g.begin_batch(););
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_6.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_6.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { let player_7 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (planned_value!("object collection Earlier", Vec::<ObjectId>::new() => q.earlier_objects(item, EarlierObjects::Exiled, Some(bound_0).into_iter().collect::<Vec<PlayerId>>()))).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
 each_8[index_6] = moved_7.clone(); all_8.extend(each_8[index_6].iter().copied());
each_9[index_6] = { let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_9.extend(each_9[index_6].iter().copied());
   } 
planned!("per-player-move-under-each-player-s-control" => g.end_batch(););
g.end_sentence();
for player in { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Indomitable Creativity", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 3, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
