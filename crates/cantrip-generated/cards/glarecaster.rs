// Glarecaster
// status: planned
// requires: engine: no once-only replacement of a whole damage event
// Flying
// {5}{W}: The next time damage would be dealt to this creature and/or you this turn, that damage is dealt to any target instead.
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("engine-no-once-only-replacement-of-a-whole-damage-event", CreateReplacement, "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"change\":{\"RedirectDamage\":{\"args\":0,\"kind\":\"Targets\"}}},\"kind\":\"ModifyEvent\"},\"result\":0},\"kind\":\"Effect\"}]}", "duration" = "\"EndOfTurn\"", "event" = "{\"Damage\":{\"grouping\":\"SourceRecipient\",\"kind\":\"Any\",\"recipients\":{\"ObjectsAndPlayers\":{\"objects\":{\"args\":{\"kind\":\"Source\"},\"kind\":\"SameObject\"},\"players\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"}}},\"source_kind\":\"Any\",\"sources\":{\"kind\":\"Any\"},\"subject\":\"Recipient\"}}", "limit" = "\"NextOccurrence\"");
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Glarecaster", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{5}{W}: The next time damage would be dealt to this creature and/or you this turn, that damage is dealt to any target instead.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
