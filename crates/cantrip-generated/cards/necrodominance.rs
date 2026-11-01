// Necrodominance
// status: planned
// requires: skipping a step of the turn
// Skip your draw step.
// At the beginning of your end step, you may pay any amount of life. If you do, draw that many cards.
// Your maximum hand size is five.
// If a card or token would be put into your graveyard from anywhere, exile it instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { let most = g.state().player(player).life.clamp(0, 255) as u32; let paid = cantrip_mechanics::choose::number(g, player, asker.source, 0, most, "Pay")?; if paid > 0 && g.may_pay(player, asker.source, asker.source, life(paid as u8), &PAY_QUESTION)? { i64::from(paid) } else { 0 } }; }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let amount_2 = { let q: &dyn Query = g; ((paid_0 as i64)).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Graveyard) && (true && Some(asker.controller) == Some(q.state().obj(id).owner)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_3_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.to = Zone::exile(q.state().obj(z.object).owner); } } }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("skipping-a-step-of-the-turn" => skip_steps: SkipStep { applies: |q, asker, player| player == asker.controller, step: Step::Draw });

pub static CARD: CardDef = CardDef { name: "Necrodominance", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, you may pay any amount of life. If you do, draw that many cards.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_3_applies, ability_3_apply, "If a card or token would be put into your graveyard from anywhere, exile it instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::MaximumHandSize(Some(5))], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
