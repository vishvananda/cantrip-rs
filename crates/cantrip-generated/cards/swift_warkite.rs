// Swift Warkite
// status: planned
// requires: cost part PlayerAction
// requires: engine: no cost moves these objects
// requires: objects that paid a cost at resolution
// Flying
// When this creature enters, you may put a creature card with mana value 3 or less from your hand or graveyard onto the battlefield. That creature gains haste. Return it to your hand at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn paid_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (3i64)) }
fn delayed_9(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_10 = { let q: &dyn Query = g; (about(q, item, 0)).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_10 = g.return_to_hand(&objects_10);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost moves these objects", Cost::NONE => Cost { moves: Some(MovesObjects { from: CostZone::Hand(Whose::Yours), to: CostDestination::Battlefield, count: CostCount::Exactly(1), filter: paid_1 }), ..Cost::NONE })), &PAY_QUESTION)? || g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("cost part PlayerAction", Cost::NONE => Cost::performed(|q: &dyn Query, asker: &Asker| Some(asker.controller), |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) <= (3i64))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::PutOntoBattlefield, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; selected_2.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let rows_3: Vec<(ObjectId, PlayerId, bool)> = objects_3.iter().map(|&o| (o, player_3, false)).collect(); let came_3 = g.put_all_onto_battlefield_as(&rows_3)?; came_3.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend(selected_2.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
 Ok(()) }))), &PAY_QUESTION)?;
let paid_objects_0: Vec<ObjectId> = planned_value!("objects that paid a cost at resolution", Vec::<ObjectId>::new() => g.objects_that_paid());
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let objects_7 = { let q: &dyn Query = g; ((paid_objects_0.clone()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let duration_7 = Duration::Indefinite;
let kind_7_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_7 { cantrip_api::instruction(g, |g| { for &object in &objects_7 { g.add_effect(source, object, kind_7_0_0, duration_7);  } }); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; paid_objects_0.clone() }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
hoisted_11.extend(objects_7.clone());
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Swift Warkite", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, you may put a creature card with mana value 3 or less from your hand or graveyard onto the battlefield. That creature gains haste. Return it to your hand at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_9, "Return it to your hand at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
