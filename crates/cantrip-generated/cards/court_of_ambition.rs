// Court of Ambition
// status: ready
// When this enchantment enters, you become the monarch.
// At the beginning of your upkeep, each opponent loses 3 life unless they discard a card. If you're the monarch, instead each opponent loses 6 life unless they discard two cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.become_monarch(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).monarch) } {
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_1_0 = vec![false; players_1.len()];
let mut each_3 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let payer_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let paid_2 = g.state().zone(Zone::hand(payer_2)).len() >= 2 && g.may_pay(payer_2, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard_cards(2)), &PAY_QUESTION)?;
 each_3[index_1] = paid_2;
   } 
g.end_sentence();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_3[index_1] as i64) != 0)) } { guard_1_0[index_1] = true;
let amount_4 = { let q: &dyn Query = g; (6i64).max(0) as i32 }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.lose_life(player, amount_4); }
   }
 } 
g.end_sentence();
} else {
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_5_0 = vec![false; players_5.len()];
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_5.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_5.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
let mut each_11 = vec![Default::default(); players_5.len()];
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { let payer_6 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_6, after_6, before_6, picked_6): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_8 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_8 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_8 = Vec::new(); for player in players_8 { cards_8.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_8, true)? }); } let discarded_8 = g.discard_together(&cards_8);
let paid_7 = !discarded_8.clone().is_empty();
(paid_7, { let q: &dyn Query = g; (discarded_8).clone() }, { let q: &dyn Query = g; (discarded_8.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_9[index_5] = after_6.clone(); all_9.extend(each_9[index_5].iter().copied());
each_10[index_5] = { let q: &dyn Query = g; before_6.clone() }; all_10.extend(each_10[index_5].iter().copied());
each_11[index_5] = paid_6;
   } 
g.end_sentence();
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_11[index_5] as i64) != 0)) } { guard_5_0[index_5] = true;
let amount_12 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_12 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_12 { g.lose_life(player, amount_12); }
   }
 } 
g.end_sentence();
hoisted_13.extend(all_9.clone());
hoisted_14.extend({ let q: &dyn Query = g; all_10.clone() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Court of Ambition", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, you become the monarch.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, each opponent loses 3 life unless they discard a card. If you're the monarch, instead each opponent loses 6 life unless they discard two cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
