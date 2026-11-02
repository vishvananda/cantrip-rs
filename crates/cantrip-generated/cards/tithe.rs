// Tithe
// status: ready
// Search your library for a Plains card. If target opponent controls more lands than you, you may search your library for an additional Plains card. Reveal those cards, put them into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn clauses_0(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let asker = &Asker::new(choice.source, q.controller(choice.source)); clauses_verdict(choice, &mut [((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains))), ((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains)))]) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) > ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) } {
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::new(cantrip_mechanics::select::up_to((1i64).max(0) as u32 + (1i64).max(0) as u32)).with(cantrip_api::select::part(&(clauses_0 as SelectFn), 0, "one for each thing searched for")))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(found_0.clone());
hoisted_6.extend(objects_1.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let searcher_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_8 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_8 = cantrip_mechanics::library::searched_cards(g, owner_8); let can_8: Vec<ObjectId> = { let q: &dyn Query = g; shown_8.iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Plains))).collect() }; let found_8 = cantrip_mechanics::library::search(g, searcher_8, owner_8, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_8, &shown_8, can_8))?;
let objects_9 = { let q: &dyn Query = g; found_8.clone() }; let mut owners_9: Vec<PlayerId> = objects_9.iter().map(|o| g.state().obj(*o).owner).collect(); owners_9.dedup(); for owner in owners_9 { let cards: Vec<ObjectId> = objects_9.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_10 = { let q: &dyn Query = g; found_8.clone() }; let moved_10 = g.return_to_hand_revealed(&objects_10);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_12.extend(moved_10.clone());
hoisted_13.extend(found_8.clone());
hoisted_14.extend(objects_9.clone());
hoisted_15.extend({ let q: &dyn Query = g; objects_10.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn clauses_fit(cards: &[ObjectId], clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> bool { let Some((&card, rest)) = cards.split_first() else { return true; }; for i in 0..clauses.len() { if clauses[i].0 > 0 && (clauses[i].1)(card) { clauses[i].0 -= 1; let fits = clauses_fit(rest, clauses); clauses[i].0 += 1; if fits { return true; } } } false }
fn clauses_verdict(choice: &Choice, clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> Verdict { let complete = clauses_fit(choice.selected, clauses); let addable = Bits::of(choice.candidates.len(), |i| { let card = choice.candidates[i]; !choice.selected.contains(&card) && { let with: Vec<ObjectId> = choice.selected.iter().copied().chain([card]).collect(); clauses_fit(&with, clauses) } }); Verdict { complete, addable } }

pub static CARD: CardDef = CardDef { name: "Tithe", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
