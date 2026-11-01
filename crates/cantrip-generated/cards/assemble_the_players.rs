// Assemble the Players
// status: ready
// You may look at the top card of your library any time.
// Once each turn, you may cast a creature spell with power 2 or less from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, card: ObjectId) -> bool { (q.chars(card).is(CardType::Creature) && q.chars(card).power.is_some_and(|v| (i64::from(v)) <= (2i64))) && !cantrip_mechanics::history::permission_used_this_turn(q, asker.source) }

pub static CARD: CardDef = CardDef { name: "Assemble the Players", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_1_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
