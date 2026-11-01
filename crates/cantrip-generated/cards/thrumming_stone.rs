// Thrumming Stone
// status: planned
// requires: player rule SpellsHaveAbilities
// Spells you cast have ripple 4. (Whenever you cast a spell, you may reveal the top four cards of your library. You may cast spells with the same name as that spell from among the revealed cards without paying their mana costs. Put the rest on the bottom of your library.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-spells-have-abilities" => statics: player_rule(ability_0_0_applies, PlayerRule::SpellsHave(SpellGrant { spells: |q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId| true, from: None, nth_each_turn: None, grants: &[SpellEffect::Ripple(4i64)] })));

pub static CARD: CardDef = CardDef { name: "Thrumming Stone", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
