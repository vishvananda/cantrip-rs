// Hushbringer
// status: ready
// Flying, lifelink
// Creatures entering or dying don't cause abilities to trigger.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && true && cantrip_mechanics::statics::entering(event).is_some_and(|id| q.chars(id).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_3_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && true && cantrip_mechanics::statics::creature_dying(q, event, triggering) }

pub static CARD: CardDef = CardDef { name: "Hushbringer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, trigger_rules: &[ability_2_trigger_rule,ability_3_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
