// Mai and Zuko
// status: ready
// Firebending 3
// You may cast Ally spells and artifact spells as though they had flash.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { player == asker.controller && (q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Ally)) || q.chars(spell).is(CardType::Artifact)) }

pub static CARD: CardDef = CardDef { name: "Mai and Zuko", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble,Subtype::Ally]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::firebending::trigger::<3>("Firebending")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, flash_rules: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
