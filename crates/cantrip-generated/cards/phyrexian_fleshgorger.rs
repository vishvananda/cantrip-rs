// Phyrexian Fleshgorger
// status: ready
// Prototype {1}{B}{B} — 3/3 (You may cast this spell with different mana cost, color, and size. It keeps its abilities and types.)
// Menace, lifelink
// Ward—Pay life equal to this creature's power.
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Phyrexian Fleshgorger", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Wurm]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward_life_equal_to_power("Ward—Pay life equal to this creature's power.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, prototype: Some(Prototype { cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, power: 3, toughness: 3, text: "Prototype {1}{B}{B} — 3/3" }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
