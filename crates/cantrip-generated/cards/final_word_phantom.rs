// Final-Word Phantom
// status: ready
// Flash
// Flying
// During each opponent's end step, you may cast spells as though they had flash.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId, player: PlayerId) -> bool { (((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| Some(bound_0) == Some(q.state().turn.active)) && q.state().turn.step() == Step::End)) && player == asker.controller && true }

pub static CARD: CardDef = CardDef { name: "Final-Word Phantom", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Detective]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, flash_rules: &[ability_2_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
