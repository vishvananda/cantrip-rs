// Traveling Chocobo
// status: ready
// You may look at the top card of your library any time.
// You may play lands and cast Bird spells from the top of your library.
// If a land or Bird you control entering the battlefield causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Bird)) }
#[allow(unused_variables)] fn ability_2_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { (triggering.permanent && Some(asker.controller) == Some(triggering.controller)) && cantrip_mechanics::statics::entering(event).is_some_and(|id| ((q.chars(id).is(CardType::Land) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Bird))) && Some(asker.controller) == Some(q.controller(id)))) }

pub static CARD: CardDef = CardDef { name: "Traveling Chocobo", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_2_trigger_rule], play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopLands,PlayRule::LibraryTopSpells(ability_1_1_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
