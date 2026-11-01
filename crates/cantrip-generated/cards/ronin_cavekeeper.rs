// Ronin Cavekeeper
// status: planned
// requires: engine: no Bushido
// Bushido 2 (Whenever this creature blocks or becomes blocked, it gets +2/+2 until end of turn.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-bushido" => abilities: cantrip_mechanics::bushido::trigger::<2>("Bushido 2"));

pub static CARD: CardDef = CardDef { name: "Ronin Cavekeeper", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Samurai]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
