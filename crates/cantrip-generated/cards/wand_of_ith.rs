// Wand of Ith
// status: ready
// {3}, {T}: Target player reveals a card at random from their hand. If it's a land card, that player discards it unless they pay 1 life. If it isn't a land card, the player discards it unless they pay life equal to its mana value. Activate only during your turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; q.state().zone(Zone::hand(player_0)).iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() };
let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut shown_0: Vec<ObjectId> = Vec::new(); while shown_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); shown_0.push(pool_0.remove(at)); }
g.reveal(player_0, &shown_0);
g.end_sentence();
if { let q: &dyn Query = g; (((shown_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let payer_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(1)), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let cards_2 = { let q: &dyn Query = g; shown_0.clone() }; let discarded_2 = g.discard_together(&cards_2);
g.end_sentence();
hoisted_3.extend(discarded_2.clone());
hoisted_4.extend({ let q: &dyn Query = g; discarded_2.clone() });
}
g.end_sentence();
hoisted_5.extend(hoisted_3.clone());
hoisted_6.extend({ let q: &dyn Query = g; hoisted_4.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; (((shown_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let payer_7 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }; let paid_7 = g.may_pay(payer_7, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { life: ({ let q: &dyn Query = g; (shown_0.clone()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))) }).clamp(0, i64::from(u32::MAX)) as u32, ..Cost::NONE }), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_7 as i64) != 0)) } {
let cards_8 = { let q: &dyn Query = g; shown_0.clone() }; let discarded_8 = g.discard_together(&cards_8);
g.end_sentence();
hoisted_9.extend(discarded_8.clone());
hoisted_10.extend({ let q: &dyn Query = g; discarded_8.clone() });
}
g.end_sentence();
hoisted_11.extend(hoisted_9.clone());
hoisted_12.extend({ let q: &dyn Query = g; hoisted_10.clone() });
}
g.end_sentence();
 Ok(()) }
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller) == Some(q.state().turn.active) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Wand of Ith", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{3}, {T}: Target player reveals a card at random from their hand. If it's a land card, that player discards it unless they pay 1 life. If it isn't a land card, the player discards it unless they pay life equal to its mana value. Activate only during your turn.").targets(ABILITY_0_TARGETS).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
