// Brighthearth Banneret
// status: planned
// requires: engine: no Reinforce
// Elemental spells and Warrior spells you cast cost {1} less to cast.
// Reinforce 1—{1}{R} ({1}{R}, Discard this card: Put a +1/+1 counter on target creature.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && (q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Elemental)) || q.chars(spell).has_subtype(q.words(asker.source).subtype(Subtype::Warrior))) { CostDelta::less((1i64).max(0) as u32) } else { CostDelta::NONE } }
planned_item!("engine-no-reinforce" => abilities: cantrip_mechanics::reinforce::ability::<1>(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Reinforce 1—{1}{R}"));

pub static CARD: CardDef = CardDef { name: "Brighthearth Banneret", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
