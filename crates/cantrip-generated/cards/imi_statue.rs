// Imi Statue
// status: planned
// requires: a limit on what untaps in an untap step
// Players can't untap more than one artifact during their untap steps.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-limit-on-what-untaps-in-an-untap-step" => untap_limits: UntapLimit { applies: |q, asker, player| true, permanents: |q, asker, id| q.chars(id).is(CardType::Artifact), amount: 1 });

pub static CARD: CardDef = CardDef { name: "Imi Statue", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
