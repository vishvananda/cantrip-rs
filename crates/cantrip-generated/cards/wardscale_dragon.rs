// Wardscale Dragon
// status: ready
// Flying
// As long as this creature is attacking, defending player can't cast spells.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !((Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().attackers.iter().any(|a| a.id == candidate) || q.was_attacking(candidate)))) && (Some(asker.source).and_then(|o: ObjectId| q.defending_player(o)).into_iter().collect::<Vec<PlayerId>>()).contains(&player) && true) }

pub static CARD: CardDef = CardDef { name: "Wardscale Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_1_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
