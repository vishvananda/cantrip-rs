// Legions of Lim-Dûl
// status: ready
// Snow swampwalk (This creature can't be blocked as long as defending player controls a snow Swamp.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_walk(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { attacker != source || !q.defending_player(attacker).is_some_and(|defending| q.state().battlefield().iter().any(|&land| q.controller(land) == defending && q.chars(land).is(CardType::Land) && (q.chars(land).supertypes.snow && q.chars(land).has_subtype(q.words(source).subtype(Subtype::Swamp))))) }

pub static CARD: CardDef = CardDef { name: "Legions of Lim-Dûl", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[ability_0_walk], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
