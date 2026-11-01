// Urza's Sylex
// status: ready
// {2}{W}{W}, {T}, Exile Urza's Sylex: Each player chooses six lands they control. Destroy all other permanents. Activate only as a sorcery.
// When Urza's Sylex is put into exile from the battlefield, you may pay {2}. If you do, search your library for a planeswalker card, reveal it, put it into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_1_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((6i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
 each_2[index_0] = selected_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!((all_2.clone()).contains(&id)) && true)).collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Exile) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_4 = g.may_pay(payer_4, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_4 as i64) != 0) } {
let searcher_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_5 = cantrip_mechanics::library::searched_cards(g, owner_5); let can_5: Vec<ObjectId> = { let q: &dyn Query = g; shown_5.iter().copied().filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect() }; let found_5 = cantrip_mechanics::library::search(g, searcher_5, owner_5, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_5, &shown_5, can_5))?;
let objects_6 = { let q: &dyn Query = g; found_5.clone() }; let mut owners_6: Vec<PlayerId> = objects_6.iter().map(|o| g.state().obj(*o).owner).collect(); owners_6.dedup(); for owner in owners_6 { let cards: Vec<ObjectId> = objects_6.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_7 = { let q: &dyn Query = g; found_5.clone() }; let moved_7 = g.return_to_hand_revealed(&objects_7);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
hoisted_9.extend(moved_7.clone());
hoisted_10.extend(found_5.clone());
hoisted_11.extend(objects_6.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Urza's Sylex", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(EXILE_THIS), ability_0, "{2}{W}{W}, {T}, Exile Urza's Sylex: Each player chooses six lands they control. Destroy all other permanents. Activate only as a sorcery.").targets(&[]).sorcery_speed(),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Urza's Sylex is put into exile from the battlefield, you may pay {2}. If you do, search your library for a planeswalker card, reveal it, put it into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
