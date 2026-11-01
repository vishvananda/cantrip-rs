// Garruk's Horde
// status: planned
// requires: a library's top card open to view
// Trample
// Play with the top card of your library revealed.
// You may cast creature spells from the top of your library. (Do this only any time you could cast that creature spell. You still pay the spell's costs.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { q.chars(spell).is(CardType::Creature) }
planned_item!("a-library-s-top-card-open-to-view" => play_rules: PlayRule::LibraryTopRevealed(|q, asker, player| player == asker.controller));

pub static CARD: CardDef = CardDef { name: "Garruk's Horde", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LibraryTopSpells(ability_2_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
