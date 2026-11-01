// Demolition Field
// status: ready
// {T}: Add {C}.
// {2}, {T}, Sacrifice this land: Destroy target nonbasic land an opponent controls. That land's controller may search their library for a basic land card, put it onto the battlefield, then shuffle. You may search your library for a basic land card, put it onto the battlefield, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::C, mana_0_0);}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_19: Vec<ObjectId> = Vec::new();
let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
if let Some(decider_2) = { let q: &dyn Query = g; (objects_1.clone()).get(0 as usize).copied().map(|o: ObjectId| q.controller(o)) } { if cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
let (carried_5_0,) = if let Some(present_5_0) = { let q: &dyn Query = g; (objects_1.clone()).get(0 as usize).copied().map(|o: ObjectId| q.controller(o)) } { if let Some(present_5_1) = { let q: &dyn Query = g; (objects_1.clone()).get(0 as usize).copied().map(|o: ObjectId| q.controller(o)) } { let searcher_4 = { let q: &dyn Query = g; present_5_0 }; let owner_4 = { let q: &dyn Query = g; present_5_1 }; let rule_count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_4 = cantrip_mechanics::library::searched_cards(g, owner_4); let can_4: Vec<ObjectId> = { let q: &dyn Query = g; shown_4.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_4 = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(rule_count_4, &shown_4, can_4))?;
(found_4.clone(),) } else { Default::default() } } else { Default::default() };
let (carried_8_0, carried_8_1,) = if let Some(present_8_0) = { let q: &dyn Query = g; (objects_1.clone()).get(0 as usize).copied().map(|o: ObjectId| q.controller(o)) } { let player_7 = { let q: &dyn Query = g; present_8_0 };
let objects_7 = ({ let q: &dyn Query = g; carried_5_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
(moved_7.clone(), { let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() },) } else { Default::default() };
for player in { let q: &dyn Query = g; (objects_1.clone()).get(0 as usize).copied().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_10.extend(carried_8_0.clone());
hoisted_11.extend(carried_5_0.clone());
hoisted_12.extend({ let q: &dyn Query = g; carried_8_1.clone() });
} }
g.end_sentence();
let decider_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_13, asker.source, Purpose::Optional)? {
let searcher_14 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_14 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_14 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_14 = cantrip_mechanics::library::searched_cards(g, owner_14); let can_14: Vec<ObjectId> = { let q: &dyn Query = g; shown_14.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_14 = cantrip_mechanics::library::search(g, searcher_14, owner_14, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(rule_count_14, &shown_14, can_14))?;
let player_15 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_15 = ({ let q: &dyn Query = g; found_14.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_15 = { let rows_15: Vec<(ObjectId, PlayerId, bool)> = objects_15.iter().map(|&o| (o, player_15, false)).collect(); let came_15 = g.put_all_onto_battlefield_as(&rows_15)?; came_15.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_17.extend(moved_15.clone());
hoisted_18.extend(found_14.clone());
hoisted_19.extend({ let q: &dyn Query = g; objects_15.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Demolition Field", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_0_mana) }, "{T}: Add {C}."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{2}, {T}, Sacrifice this land: Destroy target nonbasic land an opponent controls. That land's controller may search their library for a basic land card, put it onto the battlefield, then shuffle. You may search your library for a basic land card, put it onto the battlefield, then shuffle.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
