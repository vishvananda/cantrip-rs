// Pick the Brain
// status: ready
// Target opponent reveals their hand. You choose a nonland card from it and exile that card.
// Delirium — If there are four or more card types among cards in your graveyard, search that player's graveyard, hand, and library for any number of cards with the same name as the exiled card, exile those cards, then that player shuffles.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut owners_0: Vec<PlayerId> = objects_0.iter().map(|o| g.state().obj(*o).owner).collect(); owners_0.dedup(); for owner in owners_0 { let cards: Vec<ObjectId> = objects_0.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
if { let q: &dyn Query = g; ({ let mut seen = TypeSet::NONE; for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { for kind in q.chars(id).types.iter() { seen = seen.with(kind); } } seen.iter().count() as i64 }) >= (4i64) } {
let searcher_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") };
let mut found_3: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::graveyard(owner_3)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (moved_2.clone()).get(0 as usize).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_3, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_3.extend(taken); } }
{ let shown = g.state().zone(Zone::hand(owner_3)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (moved_2.clone()).get(0 as usize).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_3, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_3.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_3); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (moved_2.clone()).get(0 as usize).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; let taken = cantrip_mechanics::library::search(g, searcher_3, owner_3, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?; found_3.extend(taken); }
let objects_4 = { let q: &dyn Query = g; found_3.clone() }; let moved_4 = g.exile_all(&objects_4);
for player in { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_6.extend(moved_4.clone());
hoisted_7.extend(found_3.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Pick the Brain", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
