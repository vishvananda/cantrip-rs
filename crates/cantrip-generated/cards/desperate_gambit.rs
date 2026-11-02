// Desperate Gambit
// status: planned
// requires: engine: no once-only replacement of a whole damage event
// Choose a source you control and flip a coin. If you win the flip, the next time that source would deal damage this turn, it deals double that damage instead. If you lose the flip, the next time it would deal damage this turn, prevent that damage.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_4_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && cantrip_mechanics::prevention::by_chosen_source(q, affected, e) && true && Some(asker.controller) == Some(q.controller(source)) { return true; } }
 false }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; (cantrip_mechanics::prevention::damage_sources(q)).into_iter().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Choose, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_1 = g.flip_coin(player_1);
if { let q: &dyn Query = g; flip_1 } {
planned!("engine-no-once-only-replacement-of-a-whole-damage-event", CreateReplacement, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"change\":{\"SetAmount\":{\"args\":[{\"args\":2,\"kind\":\"Constant\"},{\"kind\":\"EventAmount\"}],\"kind\":\"Multiply\"}}},\"kind\":\"ModifyEvent\"},\"result\":2},\"kind\":\"Effect\"}]}", "duration" = "\"EndOfTurn\"", "event" = "{\"Damage\":{\"grouping\":\"SourceRecipient\",\"kind\":\"Any\",\"recipients\":\"Any\",\"source_kind\":\"Any\",\"sources\":{\"args\":{\"args\":{\"index\":0,\"phase\":\"Before\",\"result\":0},\"kind\":\"Result\"},\"kind\":\"SameObject\"},\"subject\":\"Recipient\"}}", "limit" = "\"NextOccurrence\"");
g.end_sentence();
} else {
{  let to = Some(Target::None); let about = { let q: &dyn Query = g; (selected_0.clone()).get(0 as usize).copied() }; if let (Some(to), Some(about)) = (to, about) { g.add_effect(item.object, about, EffectKind::Prevention { card: item.ability.card, index: 0, to, shield: Shield::NextTime }, Duration::EndOfTurn); } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Desperate Gambit", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_preventions: &[GrantedPreventionDef { hint: "Choose a source you control and flip a coin. If you win the flip, the next time that source would deal damage this turn, it deals double that damage instead. If you lose the flip, the next time it would deal damage this turn, prevent that damage.", damage: "damage", of_it: None, applies: made_4_applies, then: None, text: "If you win the flip, the next time that source would deal damage this turn, it deals double that damage instead. If you lose the flip, the next time it would deal damage this turn, prevent that damage." }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
