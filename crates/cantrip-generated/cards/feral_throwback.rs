// Feral Throwback
// status: planned
// requires: engine: no Amplify
// Amplify 2 (As this creature enters, put two +1/+1 counters on it for each Beast card you reveal in your hand.)
// Provoke (Whenever this creature attacks, you may have target creature defending player controls untap and block it if able.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-amplify" => enters: cantrip_mechanics::amplify::enters::<2>);

pub static CARD: CardDef = CardDef { name: "Feral Throwback", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::provoke::TRIGGER], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
