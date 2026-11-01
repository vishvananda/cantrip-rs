// Heat Wave
// status: planned
// requires: static rule CombatCost Block
// Cumulative upkeep {R} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// Blue creatures can't block creatures you control.
// Nonblue creatures can't block creatures you control unless their controller pays 1 life for each blocking creature they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).colors.has(q.words(asker.source).color(Color::Blue)) && (*so_far).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !((q.chars(blocker).colors.has(q.words(asker.source).color(Color::Blue)) && q.chars(blocker).is(CardType::Creature)) && (q.chars(attacker).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(attacker)))) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!((*so_far).colors.has(q.words(asker.source).color(Color::Blue))) && (*so_far).is(CardType::Creature)) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.chars(id).colors.has(q.words(asker.source).color(Color::Blue))) && q.chars(id).is(CardType::Creature)) }
planned_item!("static-rule-combat-cost-block" => block_costs: ability_2_0_rule);

pub static CARD: CardDef = CardDef { name: "Heat Wave", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), "Cumulative upkeep")], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_1_0_rule], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
