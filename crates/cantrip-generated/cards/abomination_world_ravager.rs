// Abomination, World Ravager
// status: ready
// Menace, trample
// Mayhem {4}{R} (You may cast this card from your graveyard for {4}{R} if you discarded it this turn. Timing rules still apply.)
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Abomination, World Ravager", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Gamma,Subtype::Berserker,Subtype::Villain]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::mayhem::way(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Mayhem {4}{R}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
