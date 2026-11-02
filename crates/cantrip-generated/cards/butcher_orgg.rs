// Butcher Orgg
// status: planned
// requires: static rule AssignDamageTo
// You may assign this creature's combat damage divided as you choose among defending player and/or any number of creatures they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { let source = asker.source; id == asker.source }
planned_item!("static-rule-assign-damage-to" => statics: object_rule(ability_0_0_rule, Rule::AssignDamageTo { player: Some(|q: &dyn Query, asker: &Asker| -> Option<PlayerId> { Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)) }), permanents: |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)) == Some(q.controller(id))) }));

pub static CARD: CardDef = CardDef { name: "Butcher Orgg", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 3, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Orgg]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
