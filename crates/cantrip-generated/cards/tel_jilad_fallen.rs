// Tel-Jilad Fallen
// status: ready
// Protection from artifacts
// Infect (This creature deals damage to creatures in the form of -1/-1 counters and to players in the form of poison counters.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Tel-Jilad Fallen", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Elf,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Infect]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::protection::from_type::<{ CardType::Artifact as u8 }>("Protection from Artifact")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
