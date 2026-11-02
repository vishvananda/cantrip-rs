// Auratouched Mage
// status: ready
// When this creature enters, search your library for an Aura card that could enchant it. If this creature is still on the battlefield, put that Aura card onto the battlefield attached to it. Otherwise, reveal the Aura card and put it into your hand. Then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) } {
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) && (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().any(|host: ObjectId| could_enchant(q, id, asker.controller, host)))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; found_0.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_1 { let controller = player_1; if let Some(host) = { let q: &dyn Query = g; item.subject.object() } { moved.extend(g.put_onto_battlefield_attached(o, controller, host)?); } } moved };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_3.extend(moved_1.clone());
hoisted_4.extend(found_0.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let searcher_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_6 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_6 = cantrip_mechanics::library::searched_cards(g, owner_6); let can_6: Vec<ObjectId> = { let q: &dyn Query = g; shown_6.iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) && (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().any(|host: ObjectId| could_enchant(q, id, asker.controller, host)))).collect() }; let found_6 = cantrip_mechanics::library::search(g, searcher_6, owner_6, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_6, &shown_6, can_6))?;
let objects_7 = { let q: &dyn Query = g; found_6.clone() }; let mut owners_7: Vec<PlayerId> = objects_7.iter().map(|o| g.state().obj(*o).owner).collect(); owners_7.dedup(); for owner in owners_7 { let cards: Vec<ObjectId> = objects_7.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_8 = { let q: &dyn Query = g; found_6.clone() }; let moved_8 = g.return_to_hand_revealed(&objects_8);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_10.extend(moved_8.clone());
hoisted_11.extend(found_6.clone());
hoisted_12.extend(objects_7.clone());
hoisted_13.extend({ let q: &dyn Query = g; objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn could_enchant(q: &dyn Query, aura: ObjectId, controller: PlayerId, host: ObjectId) -> bool { let chars = q.chars(aura); if chars.is(CardType::Creature) { return false; } let asker = Asker::new(aura, controller); let printed = q.card(q.state().obj(aura).card).enchant; chars.abilities().filter_map(|a| match a.index { AbilityRef::ENCHANT => q.card(a.card).enchant, AbilityRef::ENCHANT_GIVEN => q.card(a.card).becomes_aura_enchant, _ => None }).chain(printed).all(|spec| (spec.filter)(q, &asker, Target::Object(host))) && !q.chars(host).protected_from_by(&chars, || q.mana_value(aura), controller) }

pub static CARD: CardDef = CardDef { name: "Auratouched Mage", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, search your library for an Aura card that could enchant it. If this creature is still on the battlefield, put that Aura card onto the battlefield attached to it. Otherwise, reveal the Aura card and put it into your hand. Then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
