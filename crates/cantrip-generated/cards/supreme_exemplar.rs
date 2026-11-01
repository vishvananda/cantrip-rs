// Supreme Exemplar
// status: planned
// requires: engine: no Champion
// Flying
// Champion an Elemental (When this enters, sacrifice it unless you exile another Elemental you control. When this leaves the battlefield, that card returns to the battlefield.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_champion_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elemental)) }
planned_item!("engine-no-champion" => abilities: cantrip_mechanics::champion::enters(ability_1_champion_0));
planned_item!("engine-no-champion" => abilities: cantrip_mechanics::champion::LEAVES);

pub static CARD: CardDef = CardDef { name: "Supreme Exemplar", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
