// Unexpected Potential
// status: planned
// requires: a choice of CardName outside its body
// requires: card type Conspiracy
// requires: engine: no HiddenAgenda
// requires: layout Conspiracy
// Hidden agenda (Start the game with this conspiracy face down in the command zone and secretly choose a card name. You may turn this conspiracy face up any time and reveal that name.)
// You may spend mana as though it were mana of any color to cast spells with the chosen name.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, s: &Spending) -> bool { let player = s.player; player == asker.controller && s.purpose == SpendingFor::Casting && s.source.is_some_and(|spell| planned_value!("a choice of CardName outside its body", None::<CardId> => q.state().chosen_card(q.linked_to(asker))).is_some_and(|chosen| cantrip_mechanics::library::object_has_name(q, spell, chosen))) }
planned_item!("engine-no-hidden-agenda" => opening_hand: cantrip_mechanics::hidden_agenda::OPENING_HAND);
planned_item!("layout-conspiracy" => variant: Some(Variant::ConspiracyDraft));

pub static CARD: CardDef = CardDef { name: "Unexpected Potential", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Conspiracy", CardType::Land => CardType::Conspiracy)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spend_any_color: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
