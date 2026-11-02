// Great Fierce Bee
// status: planned
// requires: one or more that die read as each in the data
// Flying
// Whenever one or more other creatures die, scry 1. (Look at the top card of your library. You may put that card on the bottom.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("one-or-more-that-die-read-as-each-in-the-data", "(kind:Triggered,args:(event:ZoneChange(objects:(kind:All,args:[(kind:Not,args:(kind:SameObject,args:(kind:Source))),(kind:CardType,args:Creature)]),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Scry,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1)))))])))");

pub static CARD: CardDef = CardDef { name: "Great Fierce Bee", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Insect]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
