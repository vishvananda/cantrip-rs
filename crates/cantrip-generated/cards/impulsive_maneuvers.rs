// Impulsive Maneuvers
// status: planned
// requires: engine: no once-only replacement of a whole damage event
// Whenever a creature attacks, flip a coin. If you win the flip, the next time that creature would deal combat damage this turn, it deals double that damage instead. If you lose the flip, the next time that creature would deal combat damage this turn, prevent that damage.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if q.chars(id).is(CardType::Creature) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn made_3_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(affected) == Some(source) && true { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_0 = g.flip_coin(player_0);
if { let q: &dyn Query = g; flip_0 } {
planned!("engine-no-once-only-replacement-of-a-whole-damage-event", CreateReplacement, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"change\":{\"SetAmount\":{\"args\":[{\"args\":2,\"kind\":\"Constant\"},{\"kind\":\"EventAmount\"}],\"kind\":\"Multiply\"}}},\"kind\":\"ModifyEvent\"},\"result\":1},\"kind\":\"Effect\"}]}", "duration" = "\"EndOfTurn\"", "event" = "{\"Damage\":{\"grouping\":\"SourceRecipient\",\"kind\":\"Combat\",\"recipients\":\"Any\",\"source_kind\":\"Permanent\",\"sources\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"SameObject\"},\"subject\":\"Recipient\"}}", "limit" = "\"NextOccurrence\"");
g.end_sentence();
} else {
{  let to = Some(Target::None); let about = { let q: &dyn Query = g; item.subject.object() }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::NextTime }, Duration::EndOfTurn); } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Impulsive Maneuvers", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever a creature attacks, flip a coin. If you win the flip, the next time that creature would deal combat damage this turn, it deals double that damage instead. If you lose the flip, the next time that creature would deal combat damage this turn, prevent that damage.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Whenever a creature attacks, flip a coin. If you win the flip, the next time that creature would deal combat damage this turn, it deals double that damage instead. If you lose the flip, the next time that creature would deal combat damage this turn, prevent that damage.", damage: "damage", of_it: None, applies: made_3_applies, then: None, text: "If you win the flip, the next time that creature would deal combat damage this turn, it deals double that damage instead. If you lose the flip, the next time that creature would deal combat damage this turn, prevent that damage." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
