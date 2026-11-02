// Intervention Pact
// status: ready
// The next time a source of your choice would deal damage to you this turn, prevent that damage. You gain life equal to the damage prevented this way.
// At the beginning of your next upkeep, pay {1}{W}{W}. If you don't, you lose the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn made_2_then_steps(g: &mut dyn Game, asker: &Asker, affected: ObjectId, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let amount_3 = { let q: &dyn Query = g; ((done_prevented as i64)).min(i32::MAX.into()) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.gain_life(player, amount_3); }
 Ok(()) }
#[allow(unused_variables, dead_code)] fn made_2_then(g: &mut dyn Game, asker: &Asker, affected: ObjectId, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = made_2_then_steps(g, asker, affected, subject, data, prevented); }
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && cantrip_mechanics::prevention::by_chosen_source(q, affected, e) && true && true { return true; } }
 false }
fn delayed_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_6 = g.may_pay(payer_6, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_6 as i64) != 0)) } {
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; (cantrip_mechanics::prevention::damage_sources(q)).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Choose, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
{  let to = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>()).first().copied() }; let about = { let q: &dyn Query = g; (selected_0.clone()).get(0 as usize).copied() }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::NextTime }, Duration::EndOfTurn); } }
g.end_sentence();
g.delayed_trigger(DelayedWhen::Step { step: Step::Upkeep, on_turn_of: Some({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }) }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Intervention Pact", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_5, "The next time a source of your choice would deal damage to you this turn, prevent that damage.")], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "The next time a source of your choice would deal damage to you this turn, prevent that damage. You gain life equal to the damage prevented this way.", damage: "damage", of_it: None, applies: made_2_applies, then: Some(made_2_then), text: "The next time a source of your choice would deal damage to you this turn, prevent that damage. You gain life equal to the damage prevented this way." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
