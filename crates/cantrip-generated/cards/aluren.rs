// Aluren
// status: ready
// Any player may cast creature spells with mana value 3 or less without paying their mana costs and as though they had flash.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { true && (q.chars(spell).is(CardType::Creature) && (i64::from(q.mana_value(spell))) <= (3i64)) }

pub static CARD: CardDef = CardDef { name: "Aluren", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_alternatives: &[SpellAlternative { text: "Without paying its mana cost", applies: ability_0_0_applies, cost: InsteadOfManaCost::WithoutPaying, flash: true }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
