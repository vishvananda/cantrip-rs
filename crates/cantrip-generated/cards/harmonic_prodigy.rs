// Harmonic Prodigy
// status: ready
// Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of turn.)
// If a triggered ability of a Shaman or another Wizard you control triggers, that ability triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { (triggering.permanent && ((q.chars(triggering.source).has_subtype(q.words(asker.source).subtype(Subtype::Shaman)) || (!(Some(asker.source) == Some(triggering.source)) && q.chars(triggering.source).has_subtype(q.words(asker.source).subtype(Subtype::Wizard)))) && Some(asker.controller) == Some(triggering.controller))) && true }

pub static CARD: CardDef = CardDef { name: "Harmonic Prodigy", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::PROWESS], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_1_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
