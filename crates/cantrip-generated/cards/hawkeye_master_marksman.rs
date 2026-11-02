// Hawkeye, Master Marksman
// status: planned
// requires: mode count that is not a constant
// Reach, first strike
// Trick Arrows — Whenever Hawkeye becomes tapped, you may pay {1} up to three times. When you do, choose up to that many —
// • Net — Target creature can't block this turn.
// • Explosive — Hawkeye deals 2 damage to target player.
// • Boomerang — Discard a card, then draw a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Tapped { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_2_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static REFLEXIVE_2_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_2_mode_0_target_0)];
fn reflexive_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_3 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::CantBlock };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn reflexive_2_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static REFLEXIVE_2_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_2_mode_1_target_0)];
fn reflexive_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_4 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn reflexive_2_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_5 = Vec::new(); for player in players_5 { cards_5.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_5)?); } let discarded_5 = g.discard_together(&cards_5);
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
 Ok(()) }
static REFLEXIVE_2_MODES: ModalDef = planned_value!("mode count that is not a constant", modes(&[mode(reflexive_2_mode_0, "Net — Target creature can't block this turn.").targets(REFLEXIVE_2_MODE_0_TARGETS),mode(reflexive_2_mode_1, "Explosive — Hawkeye deals 2 damage to target player.").targets(REFLEXIVE_2_MODE_1_TARGETS),mode(reflexive_2_mode_2, "Boomerang — Discard a card, then draw a card.").targets(&[])]).choose(1, 1) => modes(&[mode(reflexive_2_mode_0, "Net — Target creature can't block this turn.").targets(REFLEXIVE_2_MODE_0_TARGETS),mode(reflexive_2_mode_1, "Explosive — Hawkeye deals 2 damage to target player.").targets(REFLEXIVE_2_MODE_1_TARGETS),mode(reflexive_2_mode_2, "Boomerang — Discard a card, then draw a card.").targets(&[])]).choose_counted(|q: &dyn Query, asker: &Asker| 0i64, |q: &dyn Query, asker: &Asker| q.told_amount(asker)));
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let most_0: u16 = { let q: &dyn Query = g; (3i64).max(0) as u16 }; let bound_0: i64 = if most_0 == 0 { 0 } else { i64::from(g.may_pay_times(payer_0, asker.source, asker.source, ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, most_0, &PAY_QUESTION)?) }; let done_0: i64 = bound_0;
if done_0 > 0 && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None, (bound_0).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Hawkeye, Master Marksman", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Archer,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Reach,Keyword::FirstStrike]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), ability_2_event), ability_2, "Trick Arrows — Whenever Hawkeye becomes tapped, you may pay {1} up to three times. When you do, choose up to that many — Net — Target creature can't block this turn; or Explosive — Hawkeye deals 2 damage to target player; or Boomerang — Discard a card, then draw a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "When you do, choose up to that many —").targets(&[]).modal(&REFLEXIVE_2_MODES)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
