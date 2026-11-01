// Invasion Plans
// status: planned
// requires: player rule Decides
// requires: static rule RequireCombat Block
// All creatures block each combat if able.
// The attacking player chooses how each creature blocks each combat.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (*so_far).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { (Some(q.state().turn.active).into_iter().collect::<Vec<PlayerId>>()).contains(&player) }
planned_item!("static-rule-require-combat-block" => must_block: ability_0_0_rule);
planned_item!("player-rule-decides" => statics: player_rule(ability_1_0_applies, PlayerRule::Decides(Decision::HowCreaturesBlock(|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).is(CardType::Creature)))));

pub static CARD: CardDef = CardDef { name: "Invasion Plans", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
