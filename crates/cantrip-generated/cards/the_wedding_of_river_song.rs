// The Wedding of River Song
// status: planned
// requires: earlier objects It
// requires: exiled card gains suspend
// Draw two cards, then you may exile a nonland card from your hand with a number of time counters on it equal to its mana value. Then target opponent does the same. Cards exiled this way that don't have suspend gain suspend.
// Time travel. (For each suspended card you own and each permanent you control with a time counter on it, you may add or remove a time counter.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
{
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { 
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = { let counters: Vec<(CounterKind, i16)> = vec![(CounterKind::Time, { let q: &dyn Query = g; ((planned_value!("earlier objects It", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::It))).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.mana_value(id)))).clamp(0, 32767) as i16 })]; let exiled = g.exile_all(&objects_2); let puts: Vec<(ObjectId, CounterKind, i16)> = exiled.iter().flat_map(|&o| counters.iter().map(move |&(kind, n)| (o, kind, n))).collect(); g.add_counters_together(&puts); exiled };
g.end_sentence();
let cards_3 = { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
planned!("exiled-card-gains-suspend" => for card in cards_3 { cantrip_mechanics::suspend::gains_suspend(g, card); });
g.end_sentence();
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(selected_1.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
let amount_7 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.draw(player, amount_7); }
g.end_sentence();
{
let selected_8_player = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::Exile, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
if !selected_8.is_empty() { 
let objects_9 = { let q: &dyn Query = g; selected_8.clone() }; let moved_9 = { let counters: Vec<(CounterKind, i16)> = vec![(CounterKind::Time, { let q: &dyn Query = g; ((planned_value!("earlier objects It", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::It))).into_iter().next().map_or(0i64, |id: ObjectId| i64::from(q.mana_value(id)))).clamp(0, 32767) as i16 })]; let exiled = g.exile_all(&objects_9); let puts: Vec<(ObjectId, CounterKind, i16)> = exiled.iter().flat_map(|&o| counters.iter().map(move |&(kind, n)| (o, kind, n))).collect(); g.add_counters_together(&puts); exiled };
g.end_sentence();
let cards_10 = { let q: &dyn Query = g; (moved_9.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
planned!("exiled-card-gains-suspend" => for card in cards_10 { cantrip_mechanics::suspend::gains_suspend(g, card); });
g.end_sentence();
hoisted_11.extend(moved_9.clone());
hoisted_12.extend(selected_8.clone());
hoisted_13.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; time_travel_for(g, player, asker.source)?; }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn time_travel_for(g: &mut dyn Game, player: PlayerId, source: ObjectId) -> Result<(), Suspend> { let mut each: Vec<ObjectId> = g.state().zone(Zone::exile(player)).iter().copied().filter(|&id| g.obj(id).counters(CounterKind::Time) > 0 && g.card(g.obj(id).card).suspend.is_some()).collect(); each.extend(g.permanents_of(player).into_iter().filter(|&id| g.obj(id).counters(CounterKind::Time) > 0)); let any = cantrip_mechanics::select::cards(cantrip_mechanics::select::ANY_NUMBER); let adding = cantrip_mechanics::choose::select(g, player, source, Purpose::PutCounters(CounterKind::Time, 1), each.clone(), any.clone())?; let rest: Vec<ObjectId> = each.into_iter().filter(|id| !adding.contains(id)).collect(); let removing = cantrip_mechanics::choose::select(g, player, source, Purpose::Choose, rest, any)?; let more: Vec<(ObjectId, CounterKind, i16)> = adding.iter().map(|&id| (id, CounterKind::Time, 1)).collect(); g.add_counters_together(&more); for id in removing { g.remove_counters(id, CounterKind::Time, 1); } Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Wedding of River Song", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
