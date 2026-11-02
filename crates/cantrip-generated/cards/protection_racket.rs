// Protection Racket
// status: ready
// At the beginning of your upkeep, repeat the following process for each opponent in turn order. Reveal the top card of your library. That player may pay life equal to that card's mana value. If they do, exile that card. Otherwise, put it into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_17: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_17: Vec<ObjectId> = Vec::new();
let mut each_18: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_18: Vec<ObjectId> = Vec::new();
let mut each_19: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_19: Vec<ObjectId> = Vec::new();
let mut each_20: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_20: Vec<ObjectId> = Vec::new();
let mut each_21: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_21: Vec<ObjectId> = Vec::new();
 for (index_0, bound_9000) in players_0.iter().copied().enumerate() { let objects_2 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let (carried_5_0,) = if let Some(present_5_0) = { let q: &dyn Query = g; Some(bound_9000) } { let payer_4 = { let q: &dyn Query = g; present_5_0 }; let paid_4 = g.may_pay(payer_4, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { life: ({ let q: &dyn Query = g; (objects_2.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))) }).clamp(0, i64::from(u32::MAX)) as u32, ..Cost::NONE }), &PAY_QUESTION)?;
(paid_4.clone(),) } else { Default::default() };
g.end_sentence();
if { let q: &dyn Query = g; ((carried_5_0 as i64) != 0) } {
let objects_6 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_6 = g.exile_all(&objects_6);
g.end_sentence();
hoisted_7.extend(moved_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let objects_9 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_9 = g.return_to_hand_revealed(&objects_9);
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_12.extend(hoisted_7.clone());
hoisted_13.extend(hoisted_10.clone());
hoisted_14.extend(objects_2.clone());
hoisted_15.extend({ let q: &dyn Query = g; hoisted_8.clone() });
hoisted_16.extend({ let q: &dyn Query = g; hoisted_11.clone() });
 each_17[index_0] = hoisted_14.clone(); all_17.extend(each_17[index_0].iter().copied());
each_18[index_0] = hoisted_12.clone(); all_18.extend(each_18[index_0].iter().copied());
each_19[index_0] = hoisted_13.clone(); all_19.extend(each_19[index_0].iter().copied());
each_20[index_0] = { let q: &dyn Query = g; hoisted_15.clone() }; all_20.extend(each_20[index_0].iter().copied());
each_21[index_0] = { let q: &dyn Query = g; hoisted_16.clone() }; all_21.extend(each_21[index_0].iter().copied());
   } 
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Protection Racket", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, repeat the following process for each opponent in turn order. Reveal the top card of your library. That player may pay life equal to that card's mana value. If they do, exile that card. Otherwise, put it into your hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
