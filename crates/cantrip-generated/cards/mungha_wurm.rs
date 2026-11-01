// Mungha Wurm
// status: planned
// requires: a limit on what untaps in an untap step
// You can't untap more than one land during your untap step.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => untap_limits: UntapLimit { applies: |q, asker, player| player == asker.controller, permanents: |q, asker, id| q.chars(id).is(CardType::Land), amount: 1 });

pub static CARD: CardDef = CardDef { name: "Mungha Wurm", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Wurm]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
