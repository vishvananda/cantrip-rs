// Pollen Remedy
// status: planned
// requires: target with a divided amount
// Kicker—Sacrifice a land. (You may sacrifice a land in addition to any other costs as you cast this spell.)
// Prevent the next 3 damage that would be dealt this turn to any number of targets, divided as you choose. If this spell was kicked, prevent the next 6 damage this way instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Land) }
static ABILITY_0_PAID_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
static ABILITY_0_UNPAID_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { if q.obj(asker.source).kicked > 0 { (cantrip_mechanics::targets::ANY.filter)(q, asker, t) } else { (cantrip_mechanics::targets::ANY.filter)(q, asker, t) } }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[TargetSpec { min: 0, max: 4294967292, filter: ability_0_target_0, label: "", if_kicked: Some((0, 4294967292)), together: None }];
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_0_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for slot in 0..item.target_count as usize { let to = g.state().target(item, slot); let shield = u16::try_from(g.state().share(item, slot)).unwrap_or(u16::MAX); if shield > 0 && g.target_legal(item, slot) { let about = to.object().unwrap_or(item.object); g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn made_4_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_0_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for slot in 0..item.target_count as usize { let to = g.state().target(item, slot); let shield = u16::try_from(g.state().share(item, slot)).unwrap_or(u16::MAX); if shield > 0 && g.target_legal(item, slot) { let about = to.object().unwrap_or(item.object); g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 1, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { if item.kicked > 0 { ability_0_paid(g, item) } else { ability_0_unpaid(g, item) } }
#[allow(unused_variables)] fn ability_0_stated_price_5(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Land) }
static ABILITY_0_STATED_PAID_TARGETS_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn made_7_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_0_stated_paid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for slot in 0..item.target_count as usize { let to = g.state().target(item, slot); let shield = u16::try_from(g.state().share(item, slot)).unwrap_or(u16::MAX); if shield > 0 && g.target_legal(item, slot) { let about = to.object().unwrap_or(item.object); g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 2, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
static ABILITY_0_STATED_UNPAID_TARGETS_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY.between(0, cantrip_api::TargetSpec::ANY_NUMBER)];
#[allow(unused_variables)] fn made_9_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && true { return true; } }
 false }
fn ability_0_stated_unpaid(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for slot in 0..item.target_count as usize { let to = g.state().target(item, slot); let shield = u16::try_from(g.state().share(item, slot)).unwrap_or(u16::MAX); if shield > 0 && g.target_legal(item, slot) { let about = to.object().unwrap_or(item.object); g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 3, to, shield: Shield::Next(shield) }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("target-with-a-divided-amount" => spell: cantrip_mechanics::cost_branch::spell("Kicker", Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_0_stated_price_5)), SpellBranch::Divided(ABILITY_0_STATED_PAID_TARGETS_TARGETS, ability_0_stated_paid, Divide { total: 6, of_x: None }), SpellBranch::Divided(ABILITY_0_STATED_UNPAID_TARGETS_TARGETS, ability_0_stated_unpaid, Divide { total: 3, of_x: None })));

pub static CARD: CardDef = CardDef { name: "Pollen Remedy", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Kicker—Sacrifice a land.", damage: "damage", of_it: None, applies: made_2_applies, then: None, text: "Kicker—Sacrifice a land." },GrantedPreventionDef { hint: "Kicker—Sacrifice a land.", damage: "damage", of_it: None, applies: made_4_applies, then: None, text: "Kicker—Sacrifice a land." },GrantedPreventionDef { hint: "Kicker—Sacrifice a land.", damage: "damage", of_it: None, applies: made_7_applies, then: None, text: "Kicker—Sacrifice a land." },GrantedPreventionDef { hint: "Kicker—Sacrifice a land.", damage: "damage", of_it: None, applies: made_9_applies, then: None, text: "Kicker—Sacrifice a land." }], labels: &["Cast without kicking", "Kicker—Sacrifice a land"], late_targets: true, optional_cost: Some(cantrip_mechanics::kicker::non_mana(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, ability_0_0)))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
