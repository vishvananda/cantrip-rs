// Future Sight
// status: planned
// requires: a library's top card open to view
// Play with the top card of your library revealed.
// You may play lands and cast spells from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { true }
planned_item!("a-library-s-top-card-open-to-view" => play_rules: PlayRule::LibraryTopRevealed(|q, asker, player| player == asker.controller));

pub static CARD: CardDef = CardDef { name: "Future Sight", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LibraryTopLands,PlayRule::LibraryTopSpells(ability_1_1_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
