// Undergrowth
// status: ready
// As an additional cost to cast this spell, you may pay {2}{R}.
// Prevent all combat damage that would be dealt this turn. If this spell's additional cost was paid, this effect doesn't affect combat damage that would be dealt by red creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_1_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && !((q.chars(source).colors.has(q.words(asker.source).color(Color::Red)) && q.chars(source).is(CardType::Creature))) && true { return true; } }
 false }
#[allow(unused_variables)] fn made_3_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && true && true && true { return true; } }
 false }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).paid > 0 || q.state().obj(candidate).kicked > 0)) } {
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
} else {
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 1, to, shield: Shield::All }, Duration::EndOfTurn); } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Undergrowth", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Prevent all combat damage that would be dealt this turn. If this spell's additional cost was paid, this effect doesn't affect combat damage that would be dealt by red creatures.", damage: "damage", of_it: None, applies: made_1_applies, then: None, text: "Prevent all combat damage that would be dealt this turn. If this spell's additional cost was paid, this effect doesn't affect combat damage that would be dealt by red creatures." },GrantedPreventionDef { hint: "Prevent all combat damage that would be dealt this turn. If this spell's additional cost was paid, this effect doesn't affect combat damage that would be dealt by red creatures.", damage: "damage", of_it: None, applies: made_3_applies, then: None, text: "Prevent all combat damage that would be dealt this turn. If this spell's additional cost was paid, this effect doesn't affect combat damage that would be dealt by red creatures." }], labels: &["Cast without paying", "Pay {2}{R}"], optional_cost: Some(OptionalCost::new(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
