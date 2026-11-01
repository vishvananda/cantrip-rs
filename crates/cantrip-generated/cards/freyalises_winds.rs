// Freyalise's Winds
// status: planned
// requires: counter kind Wind
// requires: replacement of UntapsInUntapStep
// Whenever a permanent becomes tapped, put a wind counter on it.
// If a permanent with a wind counter on it would untap during its controller's untap step, remove all wind counters from it instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Tapped { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, planned_value!("counter kind Wind", CounterKind::Charge => CounterKind::Wind), amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2]) -> Result<(), Suspend> { let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; subject.object().into_iter().collect::<Vec<ObjectId>>() };
let amount_1_0 = i16::MAX; cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += i64::from(g.remove_counters(object, planned_value!("counter kind Wind", CounterKind::Charge => CounterKind::Wind), amount_1_0)); } });
 Ok(()) }
planned_item!("replacement-of-untaps-in-untap-step" => callbacks: #[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::StepUntap(p) = proposal { let id = p.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if (i64::from(q.state().obj(id).counters(planned_value!("counter kind Wind", CounterKind::Charge => CounterKind::Wind)))) > (0i64) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::StepUntap(seen) = &seen { let p = seen; let id = p.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::StepUntap(p) = proposal { p.replaced = true; } } }
#[allow(unused_variables)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, before: &Proposal, _: &Proposal) { if let Proposal::StepUntap(p) = before { let q: &dyn Query = g; let id = p.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; let _ = ability_1_then_steps(g, asker, subject, data); } }
);
planned_item!("replacement-of-untaps-in-untap-step" => replacements: replacement(ReplacementKind::StepUntap, ability_1_applies, ability_1_apply, "If a permanent with a wind counter on it would untap during its controller's untap step, remove all wind counters from it instead.").then(ability_1_then));

pub static CARD: CardDef = CardDef { name: "Freyalise's Winds", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), ability_0_event), ability_0, "Whenever a permanent becomes tapped, put a wind counter on it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
