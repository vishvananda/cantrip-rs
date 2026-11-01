// Errant and Giada
// status: ready
// Flash
// Flying
// You may look at the top card of your library any time.
// You may cast spells with flash or flying from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { (q.chars(spell).keywords.has(Keyword::Flash) || q.chars(spell).keywords.has(Keyword::Flying)) }

pub static CARD: CardDef = CardDef { name: "Errant and Giada", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_3_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
