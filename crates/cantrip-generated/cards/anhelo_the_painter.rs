// Anhelo, the Painter
// status: planned
// requires: player rule SpellsHaveAbilities
// Deathtouch
// The first instant or sorcery spell you cast each turn has casualty 2. (As you cast that spell, you may sacrifice a creature with power 2 or greater. When you do, copy the spell and you may choose new targets for the copy.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-spells-have-abilities" => statics: player_rule(ability_1_0_applies, PlayerRule::SpellsHave(SpellGrant { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)), from: None, nth_each_turn: Some(1i64), grants: &[SpellEffect::Casualty(2i64)] })));

pub static CARD: CardDef = CardDef { name: "Anhelo, the Painter", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Assassin]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
