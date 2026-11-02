// Caradora, Heart of Alacria
// status: ready
// When Caradora enters, you may search your library for a Mount or Vehicle card, reveal it, put it into your hand, then shuffle.
// If one or more +1/+1 counters would be put on a creature or Vehicle you control, that many plus one +1/+1 counters are put on it instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.may_search(decider_0, decider_0) && cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_1 = cantrip_mechanics::library::searched_cards(g, owner_1); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Mount)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle)))).collect() }; let found_1 = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1))?;
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
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Counters(c) = proposal { let id = c.object; let subject = Target::Object(id); let data = [c.counts[CounterKind::PlusOne as usize].max(0) as u32, u32::MAX]; if c.counts[CounterKind::PlusOne as usize] > 0 && (c.entering || q.is_on_battlefield(id)) && ((q.chars(id).is(CardType::Creature) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle))) && Some(asker.controller) == Some(c.controller)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Counters(seen) = &seen { let c = seen; let id = c.object; let subject = Target::Object(id); let data = [c.counts[CounterKind::PlusOne as usize].max(0) as u32, u32::MAX]; if let Proposal::Counters(c) = proposal { c.counts[CounterKind::PlusOne as usize] = ((i64::from(data[0]) + 1i64)).clamp(0, 32767) as i16; } } }

pub static CARD: CardDef = CardDef { name: "Caradora, Heart of Alacria", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Caradora enters, you may search your library for a Mount or Vehicle card, reveal it, put it into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::Counters, ability_1_applies, ability_1_apply, "If one or more +1/+1 counters would be put on a creature or Vehicle you control, that many plus one +1/+1 counters are put on it instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
