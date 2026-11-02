// Mirror-Style Master
// status: planned
// requires: tapped token copies of several objects at once
// Backup 1 (When this creature enters, put a +1/+1 counter on target creature. If that's another creature, it gains the following ability until end of turn.)
// Whenever this creature attacks, for each attacking modified creature you control, create a tapped and attacking token that's a copy of that creature. Exile those tokens at end of combat. (Equipment, Auras you control, and counters are modifications.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_gains_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
 Ok(()) }
fn ability_0_gains_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("tapped-token-copies-of-several-objects-at-once", CreateTokens, "entry" = "{\"counters\":[],\"states\":[{\"Attacking\":{\"or_planeswalker\":false,\"player\":()}}],\"tapped\":true}", "players" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "tokens" = "[{\"amount\":{\"args\":1,\"kind\":\"Constant\"},\"recipe\":{\"CopyEach\":{\"adjustments\":[{\"args\":{\"family\":\"TokenCopyEffects\",\"form\":\"NoTokenCopyEffects\",\"operands\":[]},\"kind\":\"Term\"}],\"objects\":{\"args\":{\"args\":[{\"kind\":\"Attacking\"},{\"kind\":\"Modified\"},{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}},\"words\":\"CreatingEffect\"}]");
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0) }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn ability_0_backup(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { cantrip_mechanics::backup::resolve(g, item, 1, &[], Some(1)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_7 = { let q: &dyn Query = g; about(q, item, 0) }; let moved_7 = g.exile_all(&objects_7);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("tapped-token-copies-of-several-objects-at-once", CreateTokens, "entry" = "{\"counters\":[],\"states\":[{\"Attacking\":{\"or_planeswalker\":false,\"player\":()}}],\"tapped\":true}", "players" = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, "tokens" = "[{\"amount\":{\"args\":1,\"kind\":\"Constant\"},\"recipe\":{\"CopyEach\":{\"adjustments\":[{\"args\":{\"family\":\"TokenCopyEffects\",\"form\":\"NoTokenCopyEffects\",\"operands\":[]},\"kind\":\"Term\"}],\"objects\":{\"args\":{\"args\":[{\"kind\":\"Attacking\"},{\"kind\":\"Modified\"},{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}},\"words\":\"CreatingEffect\"}]");
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0) }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::EndOfCombat, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 2), &about); } }
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Mirror-Style Master", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::backup::trigger("Backup 1", ability_0_backup),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, for each attacking modified creature you control, create a tapped and attacking token that's a copy of that creature. Exile those tokens at end of combat.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "Backup 1"),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_gains_0_event), ability_0_gains_0, "Backup 1").targets(&[]),delayed_trigger(delayed_6, "Exile those tokens at end of combat.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
