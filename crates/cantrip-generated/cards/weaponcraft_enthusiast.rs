// Weaponcraft Enthusiast
// status: planned
// requires: engine: no Fabricate
// Fabricate 2 (When this creature enters, put two +1/+1 counters on it or create two 1/1 colorless Servo artifact creature tokens.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-fabricate" => abilities: cantrip_mechanics::fabricate::trigger::<2>("Fabricate 2"));
planned_item!("engine-no-fabricate" => extras: cantrip_mechanics::fabricate::SERVO);

pub static CARD: CardDef = CardDef { name: "Weaponcraft Enthusiast", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Aetherborn,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
