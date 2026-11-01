// Island Fish Jasconius
// status: ready
// This creature doesn't untap during your untap step.
// At the beginning of your upkeep, you may pay {U}{U}{U}. If you do, untap this creature.
// This creature can't attack unless defending player controls an Island.
// When you control no Islands, sacrifice this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_1 { g.untap(object); } });
g.end_sentence();
hoisted_2.extend(turned_1.clone());
hoisted_3.extend(objects_1.clone());
hoisted_4.extend({ let q: &dyn Query = g; turned_1.clone() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !(attacker == asker.source && (match defender { Target::Player(p) => (q.state().apnap().filter(|&bound_0: &PlayerId| !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(bound_0).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Island))).count() >= 1)).collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (q.state().apnap().filter(|&bound_0: &PlayerId| !(q.state().battlefield().iter().copied().filter(|&id| q.controller(id) == Some(bound_0).unwrap_or(PlayerId(u8::MAX)) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Island))).count() >= 1)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false })) }
fn ability_3_state(q: &dyn Query, asker: &Asker) -> bool { ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Island)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) == (0i64) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Island Fish Jasconius", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fish]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::DoesNotUntap]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, you may pay {U}{U}{U}. If you do, untap this creature.").targets(&[]),state_triggered(ability_3_state, ability_3, "When you control no Islands, sacrifice this creature.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[ability_2_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
