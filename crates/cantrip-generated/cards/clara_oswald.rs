// Clara Oswald
// status: planned
// requires: a deck rule a commander sets
// Impossible Girl — If Clara Oswald is your commander, choose a color before the game begins. Clara Oswald is the chosen color.
// If a triggered ability of a Doctor you control triggers, that ability triggers an additional time.
// Doctor's companion (You can have two commanders if the other is the Doctor.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); source == affected }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = planned_value!("depends on an instruction with a gap", None::<Color> => q.state().chosen_color(q.linked_to(asker.source))) { chars.colors = ColorSet::of(chosen); } }
#[allow(unused_variables)] fn ability_2_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && (triggering.permanent && (q.chars(triggering.source).has_subtype(q.words(asker.source).subtype(Subtype::Doctor)) && Some(asker.controller) == Some(triggering.controller))) && true }
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::ChooseAColor);

pub static CARD: CardDef = CardDef { name: "Clara Oswald", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Color, ability_1_affects, ability_1_apply_0, "If a triggered ability of a Doctor you control triggers, that ability triggers an additional time.").own().everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_2_trigger_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
