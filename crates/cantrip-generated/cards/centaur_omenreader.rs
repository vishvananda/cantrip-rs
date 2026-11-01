// Centaur Omenreader
// status: ready
// As long as this creature is tapped, creature spells you cast cost {2} less to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).tapped)) && player == asker.controller && q.chars(spell).is(CardType::Creature) { CostDelta::less((2i64) as u8) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Centaur Omenreader", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Centaur,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: false, snow: true }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
