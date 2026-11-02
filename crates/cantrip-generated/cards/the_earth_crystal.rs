// The Earth Crystal
// status: planned
// requires: target with a divided amount
// Green spells you cast cost {1} less to cast.
// If one or more +1/+1 counters would be put on a creature you control, twice that many +1/+1 counters are put on that creature instead.
// {4}{G}{G}, {T}: Distribute two +1/+1 counters among one or two target creatures you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && q.chars(spell).colors.has(q.words(asker.source).color(Color::Green)) { CostDelta::less((1i64) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::Counters(c) = proposal { let id = c.object; let subject = Target::Object(id); let data = [c.counts[CounterKind::PlusOne as usize].max(0) as u32, u32::MAX]; if c.counts[CounterKind::PlusOne as usize] > 0 && (c.entering || q.is_on_battlefield(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(c.controller)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::Counters(seen) = &seen { let c = seen; let id = c.object; let subject = Target::Object(id); let data = [c.counts[CounterKind::PlusOne as usize].max(0) as u32, u32::MAX]; if let Proposal::Counters(c) = proposal { c.counts[CounterKind::PlusOne as usize] = ((2i64 * i64::from(data[0]))).min(i16::MAX.into()) as i16; } } }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0).between(1, 2)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::counters_divided(g, item, CounterKind::PlusOne);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("target-with-a-divided-amount" => abilities: ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{4}{G}{G}, {T}: Distribute two +1/+1 counters among one or two target creatures you control.").targets(ABILITY_2_TARGETS).divide(Divide { total: 2, of_x: None }));

pub static CARD: CardDef = CardDef { name: "The Earth Crystal", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::Counters, ability_1_applies, ability_1_apply, "If one or more +1/+1 counters would be put on a creature you control, twice that many +1/+1 counters are put on that creature instead.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
