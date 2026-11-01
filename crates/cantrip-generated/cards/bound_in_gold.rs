// Bound in Gold
// status: planned
// requires: static rule Prohibit Activate
// requires: static rule Prohibit Crew
// Enchant permanent
// Enchanted permanent can't attack, block, or crew Vehicles, and its activated abilities can't be activated unless they're mana abilities.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (true)) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(attacker)) }
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(blocker) && true) }
#[allow(unused_variables)] fn ability_1_2_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; q.state().obj(asker.source).attached_to == Some(id) }
#[allow(unused_variables)] fn ability_1_3_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; q.state().obj(asker.source).attached_to == Some(id) }
planned_item!("static-rule-prohibit-crew" => statics: object_rule(ability_1_2_rule, Rule::Cant(Action::Crew)));
planned_item!("static-rule-prohibit-activate" => statics: object_rule(ability_1_3_rule, Rule::Cant(Action::Activate(|q: &dyn Query, asker: &Asker, object: ObjectId, ability: &ActivatedDef| !(ability.is_mana_ability())))));

pub static CARD: CardDef = CardDef { name: "Bound in Gold", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_1_1_rule], block_requirements: &[], attack_rules: &[ability_1_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
