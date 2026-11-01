// Inspiring Statuary
// status: ready
// Nonartifact spells you cast have improvise. (Your artifacts can help cast those spells. Each artifact you tap after you're done activating mana abilities pays for {1}.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_pay_with(q: &dyn Query, source: ObjectId, caster: PlayerId, spell: ObjectId) -> PayWith { let asker = &Asker::new(source, q.controller(source)); if Some(asker.controller) == Some(caster) && (!(q.chars(spell).is(CardType::Artifact))) { PayWith { improvise: true, ..PayWith::NONE } } else { PayWith::NONE } }

pub static CARD: CardDef = CardDef { name: "Inspiring Statuary", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, grants_pay_with: &[ability_0_0_pay_with], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
