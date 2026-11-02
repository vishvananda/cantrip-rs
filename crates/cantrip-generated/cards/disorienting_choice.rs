// Disorienting Choice
// status: ready
// For each opponent, choose up to one target artifact or enchantment that player controls. For each permanent chosen this way, its controller may exile it. Then if one or more of the chosen permanents are still on the battlefield, you search your library for up to that many land cards, put them onto the battlefield tapped, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::controlled_by_slot_opponent(q, id)) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, TargetSpec::EACH_OPPONENT)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for bound_0 in { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() } {
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5 = vec![Default::default(); players_0.len()];
 for (index_0, bound_1) in players_0.iter().copied().enumerate() { let payer_1 = { let q: &dyn Query = g; Some(bound_1).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)? {
let objects_2 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
let paid_2 = moved_2.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_2, { let q: &dyn Query = g; (moved_2).clone() }, { let q: &dyn Query = g; (objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_3[index_0] = after_1.clone(); all_3.extend(each_3[index_0].iter().copied());
each_4[index_0] = { let q: &dyn Query = g; before_1.clone() }; all_4.extend(each_4[index_0].iter().copied());
each_5[index_0] = paid_1;
   } 
g.end_sentence();
hoisted_6.extend(all_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; all_4.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; (((t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Battlefield)).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let searcher_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_8 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((((t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Battlefield).collect::<Vec<ObjectId>>()).len() as i64) as u32) }; let shown_8 = cantrip_mechanics::library::searched_cards(g, owner_8); let can_8: Vec<ObjectId> = { let q: &dyn Query = g; shown_8.iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect() }; let found_8 = cantrip_mechanics::library::search(g, searcher_8, owner_8, asker.source, SearchDestination::BattlefieldTapped, cantrip_mechanics::select::Selection::listed(rule_count_8, &shown_8, can_8))?;
let player_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_9 = ({ let q: &dyn Query = g; found_8.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_9 = { let rows_9: Vec<(ObjectId, PlayerId, bool)> = objects_9.iter().map(|&o| (o, player_9, true)).collect(); let came_9 = g.put_all_onto_battlefield_as(&rows_9)?; came_9.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_11.extend(moved_9.clone());
hoisted_12.extend(found_8.clone());
hoisted_13.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Disorienting Choice", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
