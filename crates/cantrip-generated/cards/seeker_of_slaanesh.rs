// Seeker of Slaanesh
// status: planned
// requires: player rule MustAttack
// Haste
// Allure of Slaanesh — Each opponent must attack with at least one creature each combat if able.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player != asker.controller }
planned_item!("player-rule-must-attack" => statics: player_rule(ability_1_0_applies, PlayerRule::MustAttack { attackers: |q: &dyn Query, asker: &Asker, attacker: ObjectId| q.chars(attacker).is(CardType::Creature), defenders: |q: &dyn Query, asker: &Asker, defender: Target| true, count: Some((Comparison::GreaterOrEqual, 1i64)) }));

pub static CARD: CardDef = CardDef { name: "Seeker of Slaanesh", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
