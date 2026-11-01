// Reservoir Kraken
// status: planned
// requires: a cost at resolution filtered by who is asked to pay it
// requires: player reference FirstWilling
// Trample, ward {2}
// At the beginning of each combat, if this creature is untapped, any opponent may tap an untapped creature they control. If they do, tap this creature and create a 1/1 blue Fish creature token with "This token can't be blocked."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| !(q.state().obj(candidate).tapped)) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && true { return Some(Fired { subject, data }); } }
 None }
pub mod token_8 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Fish).name, token_words: token!(Fish).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fish]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(2)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CantBeBlocked]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let willing_0: Option<PlayerId> = planned_value!("player reference FirstWilling", None::<PlayerId> => cantrip_mechanics::choose::first_willing(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }, asker.source)?);
let mut paid_3 = false; for bound_1000004 in { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() } {
let payer_5 = { let q: &dyn Query = g; Some(bound_1000004).expect("a reference that always names a player") }; let paid_5 = ({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature)) && willing_0 == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }).len() >= 1 && g.may_pay(payer_5, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("a cost at resolution filtered by who is asked to pay it", Cost::NONE => Cost::tap_state(&[Selection { objects: Picks::Matching(|q: &dyn Query, asker: &Asker, id: ObjectId| ((!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature)) && willing_0 == Some(q.controller(id)))), count: Count::Exactly(1i64) }], true))), &PAY_QUESTION)?;
if paid_5 { paid_3 = true; break; } }

g.end_sentence();
if { let q: &dyn Query = g; ((paid_3 as i64) != 0) } {
let objects_6 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_6: Vec<ObjectId> = objects_6.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_6);
g.end_sentence();
let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_7 = Vec::new();
let count_7_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_7_0 = g.card_id(&token_8::CARD);
for player in players_7 {
tokens_7.extend(std::iter::repeat((def_7_0,player)).take(count_7_0 as usize));
} let words_7 = g.words(asker.source);
let created_7 = g.create_tokens_worded(&tokens_7, false, None, words_7)?;
g.end_sentence();
hoisted_9.extend(turned_6.clone());
hoisted_10.extend(objects_6.clone());
hoisted_11.extend(created_7.clone());
hoisted_12.extend({ let q: &dyn Query = g; turned_6.clone() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Reservoir Kraken", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Kraken]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}"),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of each combat, if this creature is untapped, any opponent may tap an untapped creature they control. If they do, tap this creature and create a 1/1 blue Fish creature token with \"This token can't be blocked.\"").targets(&[]).condition(ability_2_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_8::CARD];
