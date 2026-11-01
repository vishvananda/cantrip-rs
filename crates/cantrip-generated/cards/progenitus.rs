// Progenitus
// status: planned
// requires: engine: no protection from this quality
// Protection from everything
// If Progenitus would be put into a graveyard from anywhere, reveal Progenitus and shuffle it into its owner's library instead.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-protection-from-this-quality" => statics: cantrip_mechanics::protection::from_matching(ObjectKind::Any, |q, asker, source| true, "Protection from a quality"));

pub static CARD: CardDef = CardDef { name: "Progenitus", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 2, 2, 2, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hydra,Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(10), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[cantrip_mechanics::replacements::REVEAL_AND_SHUFFLE_INTO_LIBRARY_INSTEAD_OF_GRAVEYARD], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
