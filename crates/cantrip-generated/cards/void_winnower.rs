// Void Winnower
// status: ready
// Your opponents can't cast spells with even mana values. (Zero is even.)
// Your opponents can't block with creatures with even mana values.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(player != asker.controller && ((i64::from(q.mana_value(spell))) - (0i64)) % 2 == 0) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); let player = q.controller(blocker); !(player != asker.controller && (q.chars(blocker).is(CardType::Creature) && ((i64::from(q.mana_value(blocker))) - (0i64)) % 2 == 0)) }

pub static CARD: CardDef = CardDef { name: "Void Winnower", token_words: &[], oracle: true, cost: ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(11), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_1_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
