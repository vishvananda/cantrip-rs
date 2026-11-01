// Spellweaver Eternal
// status: planned
// requires: engine: no Afflict
// Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1 until end of turn.)
// Afflict 2 (Whenever this creature becomes blocked, defending player loses 2 life.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-afflict" => abilities: cantrip_mechanics::afflict::trigger::<2>("Afflict 2"));

pub static CARD: CardDef = CardDef { name: "Spellweaver Eternal", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::Snake,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::PROWESS], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
