// Serra's Liturgy
// status: ready
// At the beginning of your upkeep, you may put a verse counter on this enchantment.
// {W}, Sacrifice this enchantment: Destroy up to X target artifacts and/or enchantments, where X is the number of verse counters on this enchantment.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::Verse, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_2.extend(applied_1.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_up_to_most(q: &dyn Query, source: ObjectId, chooser: PlayerId, x: u32) -> usize { let asker = &Asker::new(source, chooser); (((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Verse))).sum::<i64>()).max(0) as u32 as usize).max(slot_held(q, source, 0)) }
fn ability_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Verdict { let most = ability_1_0_up_to_most(q, choice.source, choice.chooser, x); Verdict { complete: choice.selected.len() <= most, addable: Bits::new(choice.candidates.len(), choice.selected.len() < most) } }
fn largest_ability_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Vec<ObjectId> { choice.candidates.iter().copied().take(ability_1_0_up_to_most(q, choice.source, choice.chooser, x)).collect() }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_api::select::set_part(&(ability_1_0_up_to as SelectFn), &(largest_ability_1_0_up_to as cantrip_api::select::LargestFn), 0, "up to X"))];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_3 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn slot_held(q: &dyn Query, source: ObjectId, slot: usize) -> usize { let s = q.state(); s.stack.iter().chain(s.left_stack.iter()).filter(|i| i.object == source).map(|i| s.targets(i)[s.spec_targets(i, slot)].iter().filter(|t| **t != Target::None).count()).max().unwrap_or(0) }

pub static CARD: CardDef = CardDef { name: "Serra's Liturgy", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, you may put a verse counter on this enchantment.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS), ability_1, "{W}, Sacrifice this enchantment: Destroy up to X target artifacts and/or enchantments, where X is the number of verse counters on this enchantment.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
