// Roaming Throne
// status: ready
// Ward {2}
// As this creature enters, choose a creature type.
// This creature is the chosen type in addition to its other types.
// If a triggered ability of another creature you control of the chosen type triggers, it triggers an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let Some(chosen_0) = cantrip_mechanics::chosen_type::choose_creature_type(g, entering)? else { return Ok(()) };
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = q.state().chosen_subtype(q.linked_to(asker.source)) { chars.subtypes = chars.subtypes.with(chosen); } }
#[allow(unused_variables)] fn ability_3_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { (triggering.permanent && (!(Some(asker.source) == Some(triggering.source)) && q.chars(triggering.source).is(CardType::Creature) && Some(asker.controller) == Some(triggering.controller) && q.state().chosen_subtype(q.linked_to(asker)).is_some_and(|chosen| q.chars(triggering.source).has_subtype(chosen)))) && true }

pub static CARD: CardDef = CardDef { name: "Roaming Throne", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Golem]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}")], statics: &[characteristic_defining(Layer::Type, ability_2_apply_0, "This creature is the chosen type in addition to its other types.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, additional_triggers: &[ability_3_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
