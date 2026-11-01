// Torpor Orb
// status: ready
// Creatures entering don't cause abilities to trigger.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && cantrip_mechanics::statics::entering(event).is_some_and(|id| q.chars(id).is(CardType::Creature)) }

pub static CARD: CardDef = CardDef { name: "Torpor Orb", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, trigger_rules: &[ability_0_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
