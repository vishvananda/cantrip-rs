// Elsha of the Infinite
// status: planned
// requires: play permission from the top of a library on other terms
// Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of turn.)
// You may look at the top card of your library any time.
// You may cast noncreature spells from the top of your library. If you cast a spell this way, you may cast it as though it had flash.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("play-permission-from-the-top-of-a-library-on-other-terms" => #[allow(unused_variables)] fn ability_2_0_cards_0(q: &dyn Query, asker: &Asker, card: ObjectId) -> bool { !(q.chars(card).is(CardType::Creature)) });
planned_item!("play-permission-from-the-top-of-a-library-on-other-terms" => play_rules: PlayRule::LibraryTopSpellsWith(ability_2_0_cards_0, LibraryTerms { additional: Cost::NONE, flash: true }));

pub static CARD: CardDef = CardDef { name: "Elsha of the Infinite", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Djinn,Subtype::Monk]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::PROWESS], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
