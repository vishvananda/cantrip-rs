// Baneslayer Angel
// status: ready
// Flying, first strike, lifelink, protection from Demons and from Dragons
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Baneslayer Angel", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Demon) }>("Protection from Demon"),cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Dragon) }>("Protection from Dragon")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
