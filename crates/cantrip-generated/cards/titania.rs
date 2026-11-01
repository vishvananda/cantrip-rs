// Titania
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// You may play an additional land on each of your turns.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 2, life: -5 }));

pub static CARD: CardDef = CardDef { name: "Titania", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::ExtraLandPlays(1)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
