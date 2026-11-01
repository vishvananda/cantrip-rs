// City of Solitude
// status: planned
// requires: player rule ActivationTiming
// Players can cast spells and activate abilities only during their own turns.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && true) || q.state().turn.active == player }
#[allow(unused_variables)] fn ability_0_1_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { true }
planned_item!("player-rule-activation-timing" => statics: player_rule(ability_0_1_applies, PlayerRule::ActivationTiming(SpellTiming::OwnTurnOnly)));

pub static CARD: CardDef = CardDef { name: "City of Solitude", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
