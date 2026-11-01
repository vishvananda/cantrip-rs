// Solemnity
// status: planned
// requires: prohibited action ReceiveCounters
// Players can't get counters.
// Counters can't be put on artifacts, creatures, enchantments, or lands.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Artifact) || (*so_far).is(CardType::Creature) || (*so_far).is(CardType::Enchantment) || (*so_far).is(CardType::Land)) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, object: ObjectId, controller: PlayerId, kind: CounterKind) -> bool { (q.chars(object).is(CardType::Artifact) || q.chars(object).is(CardType::Creature) || q.chars(object).is(CardType::Enchantment) || q.chars(object).is(CardType::Land)) && true }
planned_item!("prohibited-action-receive-counters" => statics: player_rule(ability_0_0_applies, PlayerRule::Cant(Prohibition::ReceiveCounters(None))));

pub static CARD: CardDef = CardDef { name: "Solemnity", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, counter_bans: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
