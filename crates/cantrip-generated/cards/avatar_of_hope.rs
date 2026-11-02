// Avatar of Hope
// status: planned
// requires: static rule BlockCapacity
// If you have 3 or less life, this spell costs {6} less to cast.
// Flying
// This creature can block any number of creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) <= (3i64) { CostDelta::less((6i64).max(0) as u32) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> Option<u32> { (id == asker.source).then(|| u32::MAX) }
planned_item!("static-rule-block-capacity" => additional_blocks: ability_2_0_rule);

pub static CARD: CardDef = CardDef { name: "Avatar of Hope", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
