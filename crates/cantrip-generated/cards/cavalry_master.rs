// Cavalry Master
// status: planned
// requires: keyword Flanking
// Flanking (Whenever a creature without flanking blocks this creature, the blocking creature gets -1/-1 until end of turn.)
// Other creatures you control with flanking have flanking. (Each instance of flanking triggers separately.)
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("keyword-flanking", "(kind:Static,args:(kind:Ordinary,affected:Objects((kind:Battlefield,args:(kind:All,args:[(kind:Not,args:(kind:SameObject,args:(kind:Source))),(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController))),(kind:HasKeyword,args:(kind:Flanking))])]))),changes:[(kind:GrantKeyword,args:(kind:Flanking))]))");

pub static CARD: CardDef = CardDef { name: "Cavalry Master", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::flanking::TRIGGER], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
