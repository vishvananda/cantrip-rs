// Judoon Enforcers
// status: ready
// Trample
// No more than one creature can attack you each combat.
// Suspend 6—{1}{R}{W} (Rather than cast this card from your hand, you may pay {1}{R}{W} and exile it with six time counters on it. At the beginning of your upkeep, remove a time counter. When the last is removed, you may cast it without paying its mana cost. It has haste.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_limit(q: &dyn Query, asker: &Asker, attacked: Target) -> Option<u32> { (attacked == Target::Player(asker.controller)).then_some(1) }

pub static CARD: CardDef = CardDef { name: "Judoon Enforcers", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien,Subtype::Rhino,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::suspend::ability(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Suspend")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, attack_limits: &[ability_1_limit], suspend: Some(SuspendDef { count: 6, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] } }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
