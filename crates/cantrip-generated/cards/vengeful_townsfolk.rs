// Vengeful Townsfolk
// status: planned
// requires: one or more that die read as each in the data
// Whenever one or more other creatures you control die, put a +1/+1 counter on this creature.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("one-or-more-that-die-read-as-each-in-the-data", "(kind:Triggered,args:(event:ZoneChange(objects:(kind:All,args:[(kind:Not,args:(kind:SameObject,args:(kind:Source))),(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))])]),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:Source))),kind:PowerToughness(1,1),amount:(kind:Constant,args:1),selection:None)]))))])))");

pub static CARD: CardDef = CardDef { name: "Vengeful Townsfolk", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Citizen]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
