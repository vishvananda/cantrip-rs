// Cunning Giant
// status: planned
// requires: static rule AssignDamageTo
// If this creature is unblocked, you may have it assign its combat damage to a creature defending player controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().attackers.iter().any(|a| a.id == candidate && !a.blocked))) && (source == affected) }
#[allow(unused_variables)] fn ability_0_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; (Some(asker.source).is_some_and(|candidate: ObjectId| q.state().attackers.iter().any(|a| a.id == candidate && !a.blocked))) && id == asker.source }
planned_item!("static-rule-assign-damage-to" => statics: object_rule(ability_0_0_0_rule, Rule::AssignDamageTo { player: None, permanents: |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)) == Some(q.controller(id))) }));

pub static CARD: CardDef = CardDef { name: "Cunning Giant", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
