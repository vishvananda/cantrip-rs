// The Valeyard
// status: planned
// requires: ability VillainousChoiceTwice
// requires: engine: a two-word subtype in a card's type line
// requires: player rule AdditionalVote
// If an opponent would face a villainous choice, they face that choice an additional time. (They can make the same or different choices.)
// While voting, you may vote an additional time.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("ability-villainous-choice-twice" => statics: villainous_choice_twice(|q: &dyn Query, asker: &Asker, player: PlayerId| player != asker.controller));
planned_item!("player-rule-additional-vote" => statics: player_rule(ability_1_0_applies, PlayerRule::AdditionalVote { optional: true }));

pub static CARD: CardDef = CardDef { name: "The Valeyard", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[planned_value!("engine: a two-word subtype in a card's type line", Subtype::TimeLord => Subtype::TimeLord),Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
