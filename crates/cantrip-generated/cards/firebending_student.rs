// Firebending Student
// status: planned
// requires: Firebending without a fixed number
// Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of turn.)
// Firebending X, where X is this creature's power. (Whenever this creature attacks, add X {R}. This mana lasts until end of combat.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("firebending-without-a-fixed-number" => abilities: cantrip_mechanics::firebending::trigger_by(|q: &dyn Query, asker: &Asker| -> i64 { Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))) }, "Firebending X, where X is this creature's power."));

pub static CARD: CardDef = CardDef { name: "Firebending Student", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Monk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::PROWESS], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
