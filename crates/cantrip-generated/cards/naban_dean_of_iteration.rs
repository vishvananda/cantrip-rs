// Naban, Dean of Iteration
// status: ready
// If a Wizard you control entering causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && (triggering.permanent && Some(asker.controller) == Some(triggering.controller)) && cantrip_mechanics::statics::entering(event).is_some_and(|id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Wizard)) && Some(asker.controller) == Some(q.controller(id)))) }

pub static CARD: CardDef = CardDef { name: "Naban, Dean of Iteration", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_0_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
