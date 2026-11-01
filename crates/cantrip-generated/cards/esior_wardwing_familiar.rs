// Esior, Wardwing Familiar
// status: planned
// requires: engine: no commander
// Flying
// Spells your opponents cast that target one or more commanders you control cost {3} more to cast.
// Partner (You can have two commanders if both have partner.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player != asker.controller && targets.iter().any(|t| t.object().is_some_and(|o| (q.state().battlefield().iter().copied().filter(|&id| (planned_value!("engine: no commander", false => q.is_commander(id)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).contains(&o))) { CostDelta::more(3) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Esior, Wardwing Familiar", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bird]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
