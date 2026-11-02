// Spore Cloud
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// Tap all blocking creatures. Prevent all combat damage that would be dealt this turn. Each attacking creature and each blocking creature doesn't untap during its controller's next untap step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_2_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && true { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.state().blocks.iter().any(|b| b.blocker == id) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_0: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_0);
g.end_sentence();
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature)) || (q.state().blocks.iter().any(|b| b.blocker == id) && q.chars(id).is(CardType::Creature)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep);
let kinds_3 = { let mut all = Vec::new(); for &object in &objects_3 { let kind = { let q: &dyn Query = g; planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap) }; all.push((object, kind));  } all }; if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &(object, kind) in &kinds_3 { g.add_effect(source, object, kind, duration_3); } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Spore Cloud", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Tap all blocking creatures. Prevent all combat damage that would be dealt this turn. Each attacking creature and each blocking creature doesn't untap during its controller's next untap step.", damage: "damage", of_it: None, applies: made_2_applies, then: None, text: "Prevent all combat damage that would be dealt this turn. Each attacking creature and each blocking creature doesn't untap during its controller's next untap step." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
