// Yuan Shao, the Indecisive
// status: ready
// Horsemanship (This creature can't be blocked except by creatures with horsemanship.)
// Each creature you control can't be blocked by more than one creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, id: ObjectId) -> Option<usize> { let asker = &Asker::new(source, q.controller(source)); ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).then_some(1) }

pub static CARD: CardDef = CardDef { name: "Yuan Shao, the Indecisive", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Horsemanship]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, blocker_limits: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
