// Sigarda, Host of Herons
// status: ready
// Flying, hexproof
// Spells and abilities your opponents control can't cause you to sacrifice permanents.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, causer: PlayerId, player: PlayerId, kind: CauseKind) -> bool { !(kind == CauseKind::Sacrifice && player == asker.controller && causer != asker.controller) }

pub static CARD: CardDef = CardDef { name: "Sigarda, Host of Herons", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Hexproof]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cause_rules: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
