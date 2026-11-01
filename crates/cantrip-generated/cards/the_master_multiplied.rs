// The Master, Multiplied
// status: planned
// requires: object filter OnStack
// requires: player rule CantBeCaused
// Myriad
// The "legend rule" doesn't apply to creature tokens you control.
// Triggered abilities you control can't cause you to sacrifice or exile creature tokens you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && q.state().obj(affected).token && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Creature) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))) }
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_2_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-cant-be-caused" => statics: player_rule(ability_2_0_applies, PlayerRule::CantBeCaused { action: Prohibition::Sacrifice { objects: |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id))), paying: None }, by: |q: &dyn Query, asker: &Asker, id: ObjectId| (planned_value!("object filter OnStack", false => q.stack_item(id).is_some_and(|i| i.is(StackTrait::Triggered))) && Some(asker.controller) == Some(q.controller(id))) }));
planned_item!("player-rule-cant-be-caused" => statics: player_rule(ability_2_1_applies, PlayerRule::CantBeCaused { action: Prohibition::ExileObjects(|q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id)))), by: |q: &dyn Query, asker: &Asker, id: ObjectId| (planned_value!("object filter OnStack", false => q.stack_item(id).is_some_and(|i| i.is(StackTrait::Triggered))) && Some(asker.controller) == Some(q.controller(id))) }));

pub static CARD: CardDef = CardDef { name: "The Master, Multiplied", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::TimeLord,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::myriad::TRIGGER], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::myriad::EXILE], enters: None, modal: None, enchant: None, legend_rule_exempt: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
