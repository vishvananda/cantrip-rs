// Keen-Eared Sentry
// status: planned
// requires: player rule LimitPerTurn
// You have hexproof. (You can't be the target of spells or abilities your opponents control.)
// Each opponent can't venture into the dungeon more than once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn LayerQuery, source: ObjectId, player: PlayerId) -> bool { let asker = &Asker::new(source, q.controller(source)); player == asker.controller }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller }
planned_item!("player-rule-limit-per-turn" => statics: player_rule(ability_1_0_applies, PlayerRule::LimitPerTurn(Prohibition::VentureIntoDungeon, 1i64)));

pub static CARD: CardDef = CardDef { name: "Keen-Eared Sentry", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, player_keywords: &[PlayerKeywordRule { keyword: Keyword::Hexproof, applies: ability_0_0_rule }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
