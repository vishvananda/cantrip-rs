// Field of Dreams
// status: planned
// requires: a library's top card open to view
// requires: supertype World
// Players play with the top card of their libraries revealed.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-library-s-top-card-open-to-view" => #[allow(unused_variables)] fn library_top_open_0(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true });
planned_item!("a-library-s-top-card-open-to-view" => play_rules: PlayRule::LibraryTopRevealed(library_top_open_0));

pub static CARD: CardDef = CardDef { name: "Field of Dreams", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: planned_value!("supertype World", Supertypes { basic: false, legendary: false, snow: false } => Supertypes { basic: false, legendary: false, snow: false, world: true }), colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
