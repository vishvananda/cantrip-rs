// Vizier of the Menagerie
// status: ready
// You may look at the top card of your library any time.
// You may cast creature spells from the top of your library.
// You can spend mana of any type to cast creature spells.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { q.chars(spell).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, s: &Spending) -> bool { let player = s.player; player == asker.controller && s.purpose == SpendingFor::Casting && s.source.is_some_and(|spell| q.chars(spell).is(CardType::Creature)) }

pub static CARD: CardDef = CardDef { name: "Vizier of the Menagerie", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Snake,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_1_0_rule, None)], spend_any_color: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
