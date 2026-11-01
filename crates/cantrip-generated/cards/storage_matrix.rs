// Storage Matrix
// status: planned
// requires: player rule UntapOnlyOneOfTypes
// As long as this artifact is untapped, each player chooses artifact, creature, or land during their untap step. That player can untap only permanents of the chosen type this step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| !(q.state().obj(candidate).tapped))) && true }
planned_item!("player-rule-untap-only-one-of-types" => statics: player_rule(ability_0_0_0_applies, PlayerRule::UntapOnlyOneOfTypes(&[CardType::Artifact, CardType::Creature, CardType::Land])));

pub static CARD: CardDef = CardDef { name: "Storage Matrix", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
