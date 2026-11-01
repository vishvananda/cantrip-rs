// Tomik, Distinguished Advokist
// status: planned
// requires: prohibited action PlayLand
// Flying
// Lands on the battlefield and land cards in graveyards can't be the targets of spells or abilities your opponents control.
// Your opponents can't play land cards from graveyards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Land) }
#[allow(unused_variables)] fn ability_1_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); q.chars(subject).is(CardType::Land) }
#[allow(unused_variables)] fn ability_1_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&by.controller) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).zone.kind() == ZoneKind::Graveyard && true) && (*so_far).is(CardType::Land) }
#[allow(unused_variables)] fn ability_2_0_rule_on(q: &dyn LayerQuery, source: ObjectId, subject: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(subject).zone.kind() == ZoneKind::Graveyard && true) && q.chars(subject).is(CardType::Land) }
#[allow(unused_variables)] fn ability_2_0_rule_by(q: &dyn LayerQuery, source: ObjectId, by: &Asker, by_source: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&by.controller) }
#[allow(unused_variables)] fn ability_3_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller }
planned_item!("prohibited-action-play-land" => statics: player_rule(ability_3_0_applies, PlayerRule::Cant(Prohibition::PlayLand { from: Some(Scope::Of(ZoneKind::Graveyard, |q: &dyn Query, asker: &Asker, player: PlayerId| true)), lands: |q: &dyn Query, asker: &Asker, player: PlayerId, land: ObjectId| true })));

pub static CARD: CardDef = CardDef { name: "Tomik, Distinguished Advokist", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, targeting_rules: &[TargetingRule { kind: TargetingRuleKind::Forbid(ability_1_0_rule_by), applies: ability_1_0_rule_on },TargetingRule { kind: TargetingRuleKind::Forbid(ability_2_0_rule_by), applies: ability_2_0_rule_on }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
