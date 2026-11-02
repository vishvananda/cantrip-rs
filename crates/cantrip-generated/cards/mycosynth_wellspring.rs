// Mycosynth Wellspring
// status: ready
// When this artifact enters or is put into a graveyard from the battlefield, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.may_search(decider_0, decider_0) && cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let searcher_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_1 = cantrip_mechanics::library::searched_cards(g, owner_1); let can_1: Vec<ObjectId> = { let q: &dyn Query = g; shown_1.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_1 = cantrip_mechanics::library::search(g, searcher_1, owner_1, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_1, &shown_1, can_1))?;
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
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (Some(asker.source) == Some(id) && true) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.may_search(decider_9, decider_9) && cantrip_mechanics::effects::ask_may(g, decider_9, asker.source, Purpose::Optional)? {
let searcher_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_10 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_10 = cantrip_mechanics::library::searched_cards(g, owner_10); let can_10: Vec<ObjectId> = { let q: &dyn Query = g; shown_10.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_10 = cantrip_mechanics::library::search(g, searcher_10, owner_10, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_10, &shown_10, can_10))?;
let objects_11 = { let q: &dyn Query = g; found_10.clone() }; let mut owners_11: Vec<PlayerId> = objects_11.iter().map(|o| g.state().obj(*o).owner).collect(); owners_11.dedup(); for owner in owners_11 { let cards: Vec<ObjectId> = objects_11.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_12 = { let q: &dyn Query = g; found_10.clone() }; let moved_12 = g.return_to_hand_revealed(&objects_12);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_14.extend(moved_12.clone());
hoisted_15.extend(found_10.clone());
hoisted_16.extend(objects_11.clone());
hoisted_17.extend({ let q: &dyn Query = g; objects_12.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mycosynth Wellspring", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this artifact enters or is put into a graveyard from the battlefield, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this artifact enters or is put into a graveyard from the battlefield, you may search your library for a basic land card, reveal it, put it into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
