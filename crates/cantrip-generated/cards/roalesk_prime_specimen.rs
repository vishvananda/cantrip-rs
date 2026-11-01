// Roalesk, Prime Specimen
// status: planned
// requires: operation Conjure
// Flying
// Whenever Roalesk or another permanent you control is turned face up, you may pay {X}. If you do, if X is 1 or more, conjure a random creature card with mana value X into your hand. Cloak it. It gains "{G/U}: Turn this creature face up" for as long as it remains face down.
// Disguise {G}{U}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::TurnedFaceUp { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::TurnedFaceUp { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if (!(Some(asker.source) == Some(id)) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn lasting_0(q: &dyn Query, e: &ContinuousEffect) -> bool { let asker = &Asker::new(e.source, e.controller); q.state().obj(e.affected).face_down }
fn granted_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.turn_face_up(object, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let x_0 = g.may_pay_x(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?; let paid_0 = x_0.is_some(); let bound_0: i64 = i64::from(x_0.unwrap_or(0));
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
if { let q: &dyn Query = g; (bound_0) >= (1i64) } {
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::AtRandom(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) == (bound_0))), count_1, ConjureTo::Hand)?)); }
g.end_sentence();
let controller_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let cards_2 = { let q: &dyn Query = g; (conjured_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let entered_2: Vec<ObjectId> = g.put_all_onto_battlefield_face_down(&cards_2, controller_2, FaceDown::Cloaked, false)?.into_iter().flatten().collect();
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; ((entered_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = { let q: &dyn Query = g; Duration::AsLongAs({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0) };
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
hoisted_6.extend(conjured_1.clone());
hoisted_7.extend(entered_2.clone());
hoisted_8.extend(objects_3.clone());
hoisted_9.extend(conjured_1.clone());
hoisted_10.extend({ let q: &dyn Query = g; cards_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_11.extend(hoisted_6.clone());
hoisted_12.extend(hoisted_7.clone());
hoisted_13.extend(hoisted_8.clone());
hoisted_14.extend(hoisted_9.clone());
hoisted_15.extend({ let q: &dyn Query = g; hoisted_10.clone() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Roalesk, Prime Specimen", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Mutant]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TURNED_FACE_UP.0), ability_1_event), ability_1, "Whenever Roalesk or another permanent you control is turned face up, you may pay {X}. If you do, if X is 1 or more, conjure a random creature card with mana value X into your hand. Cloak it. It gains \"{G/U}: Turn this creature face up\" for as long as it remains face down.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::G, Mana::U)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), granted_4, "{G/U}: Turn this creature face up").targets(&[])], enters: None, modal: None, enchant: None, lasting: &[Lasting { text: "its condition holds", holds: lasting_0 }], ways: &[cantrip_mechanics::disguise::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Disguise {G}{U}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
