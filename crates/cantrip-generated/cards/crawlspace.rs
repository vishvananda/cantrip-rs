// Crawlspace
// status: planned
// requires: ability AttackCap
// No more than two creatures can attack you each combat.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-attack-cap" => attack_caps: AttackCap { defender: |q: &dyn Query, asker: &Asker, defender: Target| matches!(defender, Target::Player(player) if player == asker.controller), amount: |q: &dyn Query, asker: &Asker| 2i64 });

pub static CARD: CardDef = CardDef { name: "Crawlspace", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
