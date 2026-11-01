// Tithe Taker
// status: planned
// requires: engine: no Afterlife
// During your turn, spells your opponents cast cost {1} more to cast and abilities your opponents activate cost {1} more to activate unless they're mana abilities.
// Afterlife 1 (When this creature dies, create a 1/1 white and black Spirit creature token with flying.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if (Some(asker.controller) == Some(q.state().turn.active)) && player != asker.controller && true { CostDelta::more(1) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_0_0_1_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Ability(permanent, ability, _) = costed else { return CostDelta::NONE; }; let def = q.activated_ability(ability); if (Some(asker.controller) == Some(q.state().turn.active)) && player != asker.controller && !(def.is_some_and(|d| d.is_mana_ability())) { CostDelta::more(1) } else { CostDelta::NONE } }
planned_item!("engine-no-afterlife" => abilities: cantrip_mechanics::afterlife::trigger::<1>("Afterlife 1"));
planned_item!("engine-no-afterlife" => extras: cantrip_mechanics::afterlife::SPIRIT);

pub static CARD: CardDef = CardDef { name: "Tithe Taker", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_0_rule,ability_0_0_1_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
