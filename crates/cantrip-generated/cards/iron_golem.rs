// Iron Golem
// status: planned
// requires: static rule RequireCombat Block
// Vigilance
// This creature attacks or blocks each combat if able.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_1_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { id == asker.source }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
planned_item!("static-rule-require-combat-block" => must_block: ability_1_1_rule);

pub static CARD: CardDef = CardDef { name: "Iron Golem", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Golem]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_0, "This creature attacks or blocks each combat if able.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
