// Squee
// status: planned
// requires: a hand played revealed
// requires: card type Vanguard
// requires: layout Vanguard
// Your opponents play with their hands revealed.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-hand-played-revealed" => #[allow(unused_variables)] fn hand_revealed_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller });
planned_item!("a-hand-played-revealed" => play_rules: PlayRule::HandRevealed(hand_revealed_0));
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 3, life: -4 }));

pub static CARD: CardDef = CardDef { name: "Squee", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
