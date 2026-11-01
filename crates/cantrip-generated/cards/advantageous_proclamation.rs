// Advantageous Proclamation
// status: planned
// requires: ability DraftRule
// requires: card type Conspiracy
// requires: layout Conspiracy
// (Start the game with this conspiracy face up in the command zone.)
// Your minimum deck size is reduced by five.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-draft-rule" => draft_rules: cantrip_mechanics::draft::rule(&[ConspiracyDeck::ReduceStartingDeckSize(5i64)]));
planned_item!("layout-conspiracy" => variant: Some(Variant::ConspiracyDraft));

pub static CARD: CardDef = CardDef { name: "Advantageous Proclamation", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Conspiracy", CardType::Land => CardType::Conspiracy)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
