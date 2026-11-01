// Sphinx of New Prahv
// status: ready
// Flying, vigilance
// Spells your opponents cast that target this creature cost {2} more to cast.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player != asker.controller && (true && targets.iter().any(|t| t.object().is_some_and(|o| (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).contains(&o)))) { CostDelta::more(2) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Sphinx of New Prahv", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
