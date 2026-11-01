// Glowering Rogon
// status: planned
// requires: engine: no Amplify
// Amplify 1 (As this creature enters, put a +1/+1 counter on it for each Beast card you reveal in your hand.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-amplify" => enters: cantrip_mechanics::amplify::enters::<1>);

pub static CARD: CardDef = CardDef { name: "Glowering Rogon", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
