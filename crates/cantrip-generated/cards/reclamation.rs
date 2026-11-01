// Reclamation
// status: planned
// requires: static rule CombatCost other than mana
// Black creatures can't attack unless their controller sacrifices a land of their choice for each black creature they control that's attacking. (This cost is paid as attackers are declared.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).colors.has(q.words(asker.source).color(Color::Black)) && (*so_far).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { (q.chars(attacker).colors.has(q.words(asker.source).color(Color::Black)) && q.chars(attacker).is(CardType::Creature)) && true }
planned_item!("static-rule-combat-cost-other-than-mana" => attack_cost_rules: AttackCost { applies: ability_0_0_rule, cost: "Cost" });

pub static CARD: CardDef = CardDef { name: "Reclamation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
