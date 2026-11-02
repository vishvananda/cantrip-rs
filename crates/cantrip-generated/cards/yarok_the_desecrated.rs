// Yarok, the Desecrated
// status: ready
// Deathtouch, lifelink
// If a permanent entering causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && (triggering.permanent && Some(asker.controller) == Some(triggering.controller)) && cantrip_mechanics::statics::entering(event).is_some_and(|id| true) }

pub static CARD: CardDef = CardDef { name: "Yarok, the Desecrated", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_2_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
