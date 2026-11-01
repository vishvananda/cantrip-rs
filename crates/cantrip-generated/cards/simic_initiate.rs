// Simic Initiate
// status: planned
// requires: engine: no Graft
// Graft 1 (This creature enters with a +1/+1 counter on it. Whenever another creature enters, you may move a +1/+1 counter from this creature onto it.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-graft" => enters: cantrip_mechanics::graft::enters::<1>);
planned_item!("engine-no-graft" => abilities: cantrip_mechanics::graft::TRIGGER);

pub static CARD: CardDef = CardDef { name: "Simic Initiate", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Mutant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
