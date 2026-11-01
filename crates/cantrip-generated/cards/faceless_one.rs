// Faceless One
// status: planned
// requires: a choice of Color outside its body
// requires: a deck rule a commander sets
// If Faceless One is your commander, choose a color before the game begins. Faceless One is the chosen color.
// Choose a Background (You can have a Background as a second commander.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); source == affected }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); if let Some(chosen) = planned_value!("a choice of Color outside its body", None::<Color> => q.state().chosen_color(q.linked_to(asker.source))) { chars.colors = ColorSet::of(chosen); } }
planned_item!("a-deck-rule-a-commander-sets" => deck_rules: DeckRule::ChooseAColor);

pub static CARD: CardDef = CardDef { name: "Faceless One", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Background]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Color, ability_1_affects, ability_1_apply_0, "If Faceless One is your commander, choose a color before the game begins. Faceless One is the chosen color.").own().everywhere()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
