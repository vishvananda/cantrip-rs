// Ruthless Radrat
// status: planned
// requires: Squad cost other than mana
// Squad—Exile four cards from your graveyard. (As an additional cost to cast this spell, you may pay its squad cost any number of times. When this creature enters, create that many tokens that are copies of it.)
// Menace (This creature can't be blocked except by two or more creatures.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("squad-cost-other-than-mana" => ways: cantrip_mechanics::squad::way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(exile_from_graveyard(4)), "Squad"));

pub static CARD: CardDef = CardDef { name: "Ruthless Radrat", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rat,Subtype::Mutant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
