// The Caves of Androzani
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I — Put two stun counters on each of up to two target tapped creatures.
// II, III — For each non-Saga permanent, choose a counter on it. You may put an additional counter of that kind on that permanent.
// IV — Search your library for a Doctor card, reveal it, put it into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.state().obj(id).tapped && q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0).between(0, 2)];
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Stun, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for each_object in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)))).collect::<Vec<ObjectId>>() } {
let from_2: Vec<ObjectId> = { let q: &dyn Query = g; Some(each_object).into_iter().collect::<Vec<ObjectId>>() }; let to_2: Vec<ObjectId> = { let q: &dyn Query = g; Some(each_object).into_iter().collect::<Vec<ObjectId>>() }; let kinds_2: Vec<CounterKind> = CounterKind::ALL.into_iter().filter(|&kind| true && from_2.iter().any(|&o| g.obj(o).counter_array()[kind as usize] > 0)).collect();
if !kinds_2.is_empty() { let kind = kinds_2[cantrip_mechanics::choose::one_of(g, asker.controller, asker.source, kinds_2.iter().map(|&k| Label::Counter(k)).collect())? as usize]; if cantrip_mechanics::effects::ask_may(g, asker.controller, asker.source, Purpose::Optional)? { let puts: Vec<(ObjectId, CounterKind, i16)> = to_2.iter().copied().map(|o| (o, kind, 1i16)).collect(); g.add_counters_together(&puts); } }
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for each_object in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)))).collect::<Vec<ObjectId>>() } {
let from_4: Vec<ObjectId> = { let q: &dyn Query = g; Some(each_object).into_iter().collect::<Vec<ObjectId>>() }; let to_4: Vec<ObjectId> = { let q: &dyn Query = g; Some(each_object).into_iter().collect::<Vec<ObjectId>>() }; let kinds_4: Vec<CounterKind> = CounterKind::ALL.into_iter().filter(|&kind| true && from_4.iter().any(|&o| g.obj(o).counter_array()[kind as usize] > 0)).collect();
if !kinds_4.is_empty() { let kind = kinds_4[cantrip_mechanics::choose::one_of(g, asker.controller, asker.source, kinds_4.iter().map(|&k| Label::Counter(k)).collect())? as usize]; if cantrip_mechanics::effects::ask_may(g, asker.controller, asker.source, Purpose::Optional)? { let puts: Vec<(ObjectId, CounterKind, i16)> = to_4.iter().copied().map(|o| (o, kind, 1i16)).collect(); g.add_counters_together(&puts); } }
}
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_5 = cantrip_mechanics::library::searched_cards(g, owner_5); let can_5: Vec<ObjectId> = { let q: &dyn Query = g; shown_5.iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Doctor))).collect() }; let found_5 = cantrip_mechanics::library::search(g, searcher_5, owner_5, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_5, &shown_5, can_5))?;
let objects_6 = { let q: &dyn Query = g; found_5.clone() }; let mut owners_6: Vec<PlayerId> = objects_6.iter().map(|o| g.state().obj(*o).owner).collect(); owners_6.dedup(); for owner in owners_6 { let cards: Vec<ObjectId> = objects_6.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_7 = { let q: &dyn Query = g; found_5.clone() }; let moved_7 = g.return_to_hand_revealed(&objects_7);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Caves of Androzani", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Put two stun counters on each of up to two target tapped creatures.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "II, III — For each non-Saga permanent, choose a counter on it. You may put an additional counter of that kind on that permanent.").targets(&[]),chapter(3, ability_0_2, "II, III — For each non-Saga permanent, choose a counter on it. You may put an additional counter of that kind on that permanent.").targets(&[]),chapter(4, ability_0_3, "IV — Search your library for a Doctor card, reveal it, put it into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
