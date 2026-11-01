// Djinn Illuminatus
// status: planned
// requires: engine: no cost copies a mana cost
// requires: player rule SpellsHaveAbilities
// ({U/R} can be paid with either {U} or {R}.)
// Flying
// Each instant and sorcery spell you cast has replicate. The replicate cost is equal to its mana cost. (When you cast it, copy it for each time you paid its replicate cost. You may choose new targets for the copies.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-spells-have-abilities" => statics: player_rule(ability_1_0_applies, PlayerRule::SpellsHave(SpellGrant { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)), from: None, nth_each_turn: None, grants: &[SpellEffect::Replicate(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(planned_value!("engine: no cost copies a mana cost", Cost::NONE => Cost { own_mana_cost: true, ..Cost::NONE })))] })));

pub static CARD: CardDef = CardDef { name: "Djinn Illuminatus", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::R)),Some((Mana::U, Mana::R)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Djinn]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
