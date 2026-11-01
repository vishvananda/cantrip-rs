// High Fae Trickster
// status: ready
// Flash (You may cast this spell any time you could cast an instant.)
// Flying
// You may cast spells as though they had flash.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { player == asker.controller && true }

pub static CARD: CardDef = CardDef { name: "High Fae Trickster", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, flash_rules: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
