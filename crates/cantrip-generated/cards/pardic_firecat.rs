// Pardic Firecat
// status: planned
// requires: source-zone scope on CountsAsNamed
// Haste
// If this card is in a graveyard, effects from spells named Flame Burst count it as a card named Flame Burst.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("source-zone-scope-on-counts-as-named" => statics: cantrip_mechanics::zones::scoped(&[EventZone::Graveyard], None, cantrip_mechanics::zones::counts_as_named("Flame Burst", "Flame Burst")));

pub static CARD: CardDef = CardDef { name: "Pardic Firecat", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
