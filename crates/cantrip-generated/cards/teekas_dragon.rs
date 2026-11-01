// Teeka's Dragon
// status: planned
// requires: engine: no Rampage
// Flying; trample; rampage 4 (Whenever this creature becomes blocked, it gets +4/+4 until end of turn for each creature blocking it beyond the first.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-rampage" => abilities: cantrip_mechanics::rampage::trigger::<4>("Rampage 4"));

pub static CARD: CardDef = CardDef { name: "Teeka's Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
