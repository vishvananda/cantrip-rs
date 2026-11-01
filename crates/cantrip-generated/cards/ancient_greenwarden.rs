// Ancient Greenwarden
// status: ready
// Reach (This creature can block creatures with flying.)
// You may play lands from your graveyard.
// If a land entering causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { (triggering.permanent && Some(asker.controller) == Some(triggering.controller)) && cantrip_mechanics::statics::entering(event).is_some_and(|id| q.chars(id).is(CardType::Land)) }

pub static CARD: CardDef = CardDef { name: "Ancient Greenwarden", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Reach]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_2_trigger_rule], play_rules: &[PlayRule::LandsFromGraveyard], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
