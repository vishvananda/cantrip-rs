// Zabaz, the Glimmerwasp
// status: planned
// requires: object filter OnStack
// requires: replacement event Caused
// Modular 1
// If a modular triggered ability would put one or more +1/+1 counters on a creature you control, that many plus one +1/+1 counters are put on it instead.
// {R}: Destroy target artifact you control.
// {W}: Zabaz gains flying until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_modular(_: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let kind = CounterKind::PlusOne as usize; entering.counters[kind] = entering.counters[kind].saturating_add(1); Ok(()) }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Counters(c) = proposal { let id = c.object; let subject = Target::Object(id); let data = [c.counts[CounterKind::PlusOne as usize].max(0) as u32, u32::MAX]; if c.counts[CounterKind::PlusOne as usize] > 0 && (c.entering || q.is_on_battlefield(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(c.controller)) && planned_value!("replacement event Caused", true => q.proposal_cause().is_some_and(|cause| (planned_value!("object filter OnStack", false => q.stack_item(cause).is_some_and(|i| i.is(StackTrait::Keyword))) && planned_value!("object filter OnStack", false => q.stack_item(cause).is_some_and(|i| i.is(StackTrait::Triggered)))))) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Counters(seen) = &seen { let c = seen; let id = c.object; let subject = Target::Object(id); let data = [c.counts[CounterKind::PlusOne as usize].max(0) as u32, u32::MAX]; if let Proposal::Counters(c) = proposal { c.counts[CounterKind::PlusOne as usize] = ((i64::from(data[0]) + 1i64)).min(i16::MAX.into()) as i16; } } }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying])) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Zabaz, the Glimmerwasp", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Insect]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::modular::TRIGGER,ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{R}: Destroy target artifact you control.").targets(ABILITY_2_TARGETS),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{W}: Zabaz gains flying until end of turn.").targets(&[])], statics: &[], replacements: &[replacement(ReplacementKind::Counters, ability_1_applies, ability_1_apply, "If a modular triggered ability would put one or more +1/+1 counters on a creature you control, that many plus one +1/+1 counters are put on it instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_modular), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
