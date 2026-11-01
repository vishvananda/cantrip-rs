// Dakkon Blackblade Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: player rule Unique
// You may play any colored card from your hand as a copy of a basic land card chosen at random that can produce mana of one of the card's colors.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-unique" => statics: player_rule(ability_0_0_applies, PlayerRule::ColoredCardsAsRandomBasicLands));
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: 0 }));

pub static CARD: CardDef = CardDef { name: "Dakkon Blackblade Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
