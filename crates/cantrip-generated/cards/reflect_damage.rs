// Reflect Damage
// status: planned
// requires: engine: no once-only replacement of a whole damage event
// The next time a source of your choice would deal damage this turn, that damage is dealt to that source's controller instead.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; (cantrip_mechanics::prevention::damage_sources(q)).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Choose, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
planned!("engine-no-once-only-replacement-of-a-whole-damage-event", CreateReplacement, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"change\":{\"RedirectDamage\":{\"args\":{\"args\":{\"args\":{\"args\":\"Source\",\"kind\":\"Damage\"},\"kind\":\"ControllerOf\"},\"kind\":\"One\"},\"kind\":\"Players\"}}},\"kind\":\"ModifyEvent\"},\"result\":1},\"kind\":\"Effect\"}]}", "duration" = "\"EndOfTurn\"", "event" = "{\"Damage\":{\"grouping\":\"SourceRecipient\",\"kind\":\"Any\",\"recipients\":\"Any\",\"source_kind\":\"Any\",\"sources\":{\"args\":{\"args\":{\"index\":0,\"phase\":\"Before\",\"result\":0},\"kind\":\"Result\"},\"kind\":\"SameObject\"},\"subject\":\"Recipient\"}}", "limit" = "\"NextOccurrence\"");
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Reflect Damage", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
