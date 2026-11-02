// Goblin Psychopath
// status: planned
// requires: engine: no once-only replacement of a whole damage event
// Whenever this creature attacks or blocks, flip a coin. If you lose the flip, the next time it would deal combat damage this turn, it deals that damage to you instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_0 = g.flip_coin(player_0);
if { let q: &dyn Query = g; !flip_0 } {
planned!("engine-no-once-only-replacement-of-a-whole-damage-event", CreateReplacement, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"change\":{\"RedirectDamage\":{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"Players\"}}},\"kind\":\"ModifyEvent\"},\"result\":1},\"kind\":\"Effect\"}]}", "duration" = "\"EndOfTurn\"", "event" = "{\"Damage\":{\"grouping\":\"SourceRecipient\",\"kind\":\"Combat\",\"recipients\":\"Any\",\"source_kind\":\"Permanent\",\"sources\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"SameObject\"},\"subject\":\"Recipient\"}}", "limit" = "\"NextOccurrence\"");
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_2 = g.flip_coin(player_2);
if { let q: &dyn Query = g; !flip_2 } {
planned!("engine-no-once-only-replacement-of-a-whole-damage-event", CreateReplacement, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"change\":{\"RedirectDamage\":{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"kind\":\"Players\"}}},\"kind\":\"ModifyEvent\"},\"result\":1},\"kind\":\"Effect\"}]}", "duration" = "\"EndOfTurn\"", "event" = "{\"Damage\":{\"grouping\":\"SourceRecipient\",\"kind\":\"Combat\",\"recipients\":\"Any\",\"source_kind\":\"Permanent\",\"sources\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"SameObject\"},\"subject\":\"Recipient\"}}", "limit" = "\"NextOccurrence\"");
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Goblin Psychopath", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin,Subtype::Mutant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks or blocks, flip a coin. If you lose the flip, the next time it would deal combat damage this turn, it deals that damage to you instead.").targets(&[]),triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_1_event), ability_1, "Whenever this creature attacks or blocks, flip a coin. If you lose the flip, the next time it would deal combat damage this turn, it deals that damage to you instead.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
