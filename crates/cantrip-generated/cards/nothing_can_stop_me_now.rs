// Nothing Can Stop Me Now
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: operation AbandonScheme
// requires: supertype Ongoing
// (An ongoing scheme remains face up until it's abandoned.)
// If a source an opponent controls would deal damage to you, prevent 1 of that damage.
// At the beginning of each end step, if you've been dealt 5 or more damage this turn, abandon this scheme.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(source)) && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return true; } }
 false }
#[allow(unused_variables)] fn ability_0_amount(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> u16 { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; (1i64).max(0) as u16 }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()) >= (5i64) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-abandon-scheme" => cantrip_mechanics::scheme::abandon(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source));
g.end_sentence();
 Ok(()) }
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Nothing Can Stop Me Now", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype Ongoing", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, ongoing: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each end step, if you've been dealt 5 or more damage this turn, abandon this scheme.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "If a source an opponent controls would deal damage to you, prevent 1 of that damage.", applies: ability_0_applies, amount: Some(ability_0_amount) }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
