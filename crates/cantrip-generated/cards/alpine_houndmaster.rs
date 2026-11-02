// Alpine Houndmaster
// status: ready
// When this creature enters, you may search your library for a card named Alpine Watchdog and/or a card named Igneous Cur, reveal them, put them into your hand, then shuffle.
// Whenever this creature attacks, it gets +X/+0 until end of turn, where X is the number of other attacking creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn clauses_1(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let asker = &Asker::new(choice.source, q.controller(choice.source)); clauses_verdict(choice, &mut [((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).name == "Alpine Watchdog"), ((1i64).max(0) as u32, &|id: ObjectId| q.chars(id).name == "Igneous Cur")]) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.may_search(decider_0, decider_0) && cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let found_1 = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::new(cantrip_mechanics::select::up_to((1i64).max(0) as u32 + (1i64).max(0) as u32)).with(cantrip_api::select::part(&(clauses_1 as SelectFn), 0, "one for each thing searched for")))?;
let objects_2 = { let q: &dyn Query = g; found_1.clone() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_3 = { let q: &dyn Query = g; found_1.clone() }; let moved_3 = g.return_to_hand_revealed(&objects_3);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_5.extend(moved_3.clone());
hoisted_6.extend(found_1.clone());
hoisted_7.extend(objects_2.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_9 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let duration_9 = Duration::EndOfTurn;
let kind_9_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((q.state().battlefield().iter().copied().filter(|&id| (!(item.subject.object() == Some(id)) && (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16, (0i64) as i16) };
if let Some(source) = source_9 { cantrip_api::instruction(g, |g| { for &object in &objects_9 { g.add_effect(source, object, kind_9_0_0, duration_9);  } }); }
g.end_sentence();
 Ok(()) }
fn clauses_fit(cards: &[ObjectId], clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> bool { let Some((&card, rest)) = cards.split_first() else { return true; }; for i in 0..clauses.len() { if clauses[i].0 > 0 && (clauses[i].1)(card) { clauses[i].0 -= 1; let fits = clauses_fit(rest, clauses); clauses[i].0 += 1; if fits { return true; } } } false }
fn clauses_verdict(choice: &Choice, clauses: &mut [(u32, &dyn Fn(ObjectId) -> bool)]) -> Verdict { let complete = clauses_fit(choice.selected, clauses); let addable = Bits::of(choice.candidates.len(), |i| { let card = choice.candidates[i]; !choice.selected.contains(&card) && { let with: Vec<ObjectId> = choice.selected.iter().copied().chain([card]).collect(); clauses_fit(&with, clauses) } }); Verdict { complete, addable } }

pub static CARD: CardDef = CardDef { name: "Alpine Houndmaster", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you may search your library for a card named Alpine Watchdog and/or a card named Igneous Cur, reveal them, put them into your hand, then shuffle.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, it gets +X/+0 until end of turn, where X is the number of other attacking creatures.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
