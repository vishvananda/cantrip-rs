// Winter, Cynical Opportunist
// status: planned
// requires: cost selection rule DistinctCount
// Deathtouch
// Whenever Winter attacks, mill three cards.
// Delirium — At the beginning of your end step, you may exile any number of cards from your graveyard with four or more card types among them. If you do, put a permanent card from among them onto the battlefield with a finality counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn paid_2(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { true }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost selection rule DistinctCount", Cost::NONE => Cost::moves(&[Selection { objects: Picks::Matching(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && true) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))), count: Count::AnyNumber }], Destination::Exile, Arrival { tapped: false, face_down: false, states: &[] }).such_that(&[SelectionRule::DistinctCount(Property::CardTypes, Comparison::GreaterOrEqual, 4i64)]))), &PAY_QUESTION)?;
let paid_objects_1: Vec<ObjectId> = planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => g.objects_that_paid());
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && (paid_objects_1.clone()).contains(&id))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutOntoBattlefield, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; selected_3.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, player_4, false)).collect(); let counters = [(CounterKind::Finality, { let q: &dyn Query = g; (1i64).max(0) as i16 })]; g.put_all_onto_battlefield_with_counters(&rows_4, &counters)? };
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend(selected_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Winter, Cynical Opportunist", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warlock]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Winter attacks, mill three cards.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "Delirium — At the beginning of your end step, you may exile any number of cards from your graveyard with four or more card types among them. If you do, put a permanent card from among them onto the battlefield with a finality counter on it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
