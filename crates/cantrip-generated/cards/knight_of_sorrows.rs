// Knight of Sorrows
// status: planned
// requires: engine: no Afterlife
// requires: static rule BlockCapacity
// This creature can block an additional creature each combat.
// Afterlife 1 (When this creature dies, create a 1/1 white and black Spirit creature token with flying.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> Option<u32> { (id == asker.source).then(|| (1i64).max(0) as u32) }
planned_item!("static-rule-block-capacity" => additional_blocks: ability_0_0_rule);
planned_item!("engine-no-afterlife" => abilities: cantrip_mechanics::afterlife::trigger::<1>("Afterlife 1"));
planned_item!("engine-no-afterlife" => extras: cantrip_mechanics::afterlife::SPIRIT);

pub static CARD: CardDef = CardDef { name: "Knight of Sorrows", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
