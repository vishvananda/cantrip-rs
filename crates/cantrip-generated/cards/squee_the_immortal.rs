// Squee, the Immortal
// status: ready
// You may cast this card from your graveyard or from exile.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { card == asker.source && player == asker.controller }

pub static CARD: CardDef = CardDef { name: "Squee, the Immortal", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Goblin]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CastFromYourGraveyard]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, exile_casts: &[ExileCast { text: "Cast from exile", allows: ability_1_allows, additional: Cost::NONE, any_type: false, lands: false, life_for_mana_value: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
