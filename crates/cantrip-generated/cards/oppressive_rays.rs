// Oppressive Rays
// status: planned
// requires: static rule CombatCost Block
// Enchant creature
// Enchanted creature can't attack or block unless its controller pays {3}.
// Activated abilities of enchanted creature cost {3} more to activate.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> Option<ManaCost> { (q.state().obj(asker.source).attached_to == Some(attacker) && true).then(|| ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Ability(permanent, ability, _) = costed else { return CostDelta::NONE; }; let def = q.activated_ability(ability); if true && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(permanent)) { CostDelta::more(3) } else { CostDelta::NONE } }
planned_item!("static-rule-combat-cost-block" => block_costs: ability_1_0_rule);

pub static CARD: CardDef = CardDef { name: "Oppressive Rays", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), attack_costs: &[ability_1_1_rule], cost_changes: &[ability_2_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
