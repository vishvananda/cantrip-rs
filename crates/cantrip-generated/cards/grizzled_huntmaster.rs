// Grizzled Huntmaster
// status: planned
// requires: operation Conjure
// When this creature enters, you may exile a creature card from your hand. If you do, search your hand and library for any number of cards with the same name, exile them, then shuffle. Choose a creature card you own from outside the game. Conjure a duplicate of that card into your hand for each card exiled from your hand this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Exile, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let objects_1 = { let q: &dyn Query = g; selected_2.clone() }; let moved_1 = g.exile_all(&objects_1);
let paid_1 = moved_1.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_1, { let q: &dyn Query = g; (moved_1).clone() }, { let q: &dyn Query = g; (objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_2).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let searcher_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let mut found_3: Vec<ObjectId> = Vec::new();
{ let shown = g.state().zone(Zone::hand(owner_3)).to_vec(); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (after_0.clone()).get(0).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; if !can.is_empty() { let taken = cantrip_mechanics::choose::select(g, searcher_3, asker.source, Purpose::Exile, shown.clone(), cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?;  found_3.extend(taken); } }
{ let shown = cantrip_mechanics::library::searched_cards(g, owner_3); let can: Vec<ObjectId> = { let q: &dyn Query = g; shown.iter().copied().filter(|&id| (after_0.clone()).get(0).copied().is_some_and(|o: ObjectId| cantrip_mechanics::same_name(q, id, o))).collect() }; let taken = cantrip_mechanics::library::search(g, searcher_3, owner_3, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(cantrip_mechanics::select::Count::AnyNumber, &shown, can))?; found_3.extend(taken); }
let objects_4 = { let q: &dyn Query = g; found_3.clone() }; let moved_4 = g.exile_all(&objects_4);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let duplicated_6 = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() };
let count_6: u32 = { let q: &dyn Query = g; (((objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).zone.kind() == ZoneKind::Hand).collect::<Vec<ObjectId>>()).len() as i64) as u32 }; let mut conjured_6: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_6.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_6), count_6, ConjureTo::Hand)?)); }
g.end_sentence();
hoisted_7.extend(moved_4.clone());
hoisted_8.extend(conjured_6.clone());
hoisted_9.extend(found_3.clone());
hoisted_10.extend(conjured_6.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Grizzled Huntmaster", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you may exile a creature card from your hand. If you do, search your hand and library for any number of cards with the same name, exile them, then shuffle. Choose a creature card you own from outside the game. Conjure a duplicate of that card into your hand for each card exiled from your hand this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
