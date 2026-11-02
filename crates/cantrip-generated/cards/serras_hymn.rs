// Serra's Hymn
// status: planned
// requires: target with a divided amount
// requires: target with a divided amount read as chosen
// At the beginning of your upkeep, you may put a verse counter on this enchantment.
// Sacrifice this enchantment: Prevent the next X damage that would be dealt this turn to any number of targets, divided as you choose, where X is the number of verse counters on this enchantment.
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
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(1, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn made_4_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for slot in 0..item.target_count as usize { let to = g.state().target(item, slot); let shield = u16::try_from(g.state().share(item, slot)).unwrap_or(u16::MAX); if shield > 0 && g.target_legal(item, slot) { let about = to.object().unwrap_or(item.object); g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("target-with-a-divided-amount" => abilities: ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS), ability_1, "Sacrifice this enchantment: Prevent the next X damage that would be dealt this turn to any number of targets, divided as you choose, where X is the number of verse counters on this enchantment.").targets(ABILITY_1_TARGETS).divide(Divide { total: planned_value!("target with a divided amount read as chosen", 1u32 => Divide::read(|q: &dyn Query, asker: &Asker| ((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Verse))).sum::<i64>()).max(0) as u32)), of_x: None }));

pub static CARD: CardDef = CardDef { name: "Serra's Hymn", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, you may put a verse counter on this enchantment.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Sacrifice this enchantment: Prevent the next X damage that would be dealt this turn to any number of targets, divided as you choose, where X is the number of verse counters on this enchantment.", damage: "damage", of_it: None, applies: made_4_applies, then: None, text: "Prevent the next X damage that would be dealt this turn to any number of targets, divided as you choose, where X is the number of verse counters on this enchantment." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
