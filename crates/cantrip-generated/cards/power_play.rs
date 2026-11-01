// Power Play
// status: planned
// requires: ability Unique
// requires: card type Conspiracy
// requires: layout Conspiracy
// (Start the game with this conspiracy face up in the command zone.)
// You are the starting player. If multiple players would be the starting player, one of those players is chosen at random.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-unique" => statics: game_rule(GameRule::StartingPlayer));
planned_item!("layout-conspiracy" => variant: Some(Variant::ConspiracyDraft));

pub static CARD: CardDef = CardDef { name: "Power Play", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Conspiracy", CardType::Land => CardType::Conspiracy)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
