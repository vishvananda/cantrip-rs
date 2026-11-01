// Elderwood Scion
// status: ready
// Trample, lifelink
// Spells you cast that target this creature cost {2} less to cast.
// Spells your opponents cast that target this creature cost {2} more to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && targets.iter().any(|t| t.object().is_some_and(|o| (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).contains(&o))) { CostDelta::less((2i64) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player != asker.controller && (true && targets.iter().any(|t| t.object().is_some_and(|o| (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).contains(&o)))) { CostDelta::more(2) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Elderwood Scion", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_2_0_rule,ability_3_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
