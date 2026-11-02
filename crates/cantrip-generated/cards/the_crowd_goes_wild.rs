// The Crowd Goes Wild
// status: ready
// Assist (Another player can pay up to {X} of this spell's cost. You choose the value of X.)
// Support X. (Put a +1/+1 counter on each of up to X target creatures.)
// Each creature with a +1/+1 counter on it gains trample until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_up_to_most(q: &dyn Query, source: ObjectId, chooser: PlayerId, x: u32) -> usize { let asker = &Asker::new(source, chooser); (i64::from(x)) as u32 as usize }
fn ability_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Verdict { let most = ability_1_0_up_to_most(q, choice.source, choice.chooser, x); Verdict { complete: choice.selected.len() <= most, addable: Bits::new(choice.candidates.len(), choice.selected.len() < most) } }
fn largest_ability_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Vec<ObjectId> { choice.candidates.iter().copied().take(ability_1_0_up_to_most(q, choice.source, choice.chooser, x)).collect() }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_api::select::set_part(&(ability_1_0_up_to as SelectFn), &(largest_ability_1_0_up_to as cantrip_api::select::LargestFn), TargetSpec::COUNTED, "up to X"))];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (i64::from(q.state().obj(id).counters(CounterKind::PlusOne))) > (0i64))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Trample])) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Crowd Goes Wild", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, pay_with: PayWith { assist: true, ..PayWith::NONE }, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
