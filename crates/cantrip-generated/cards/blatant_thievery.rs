// Blatant Thievery
// status: planned
// requires: target cardinality PerPlayer
// For each opponent, gain control of target permanent that player controls.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("target-cardinality-per-player", "(kind:Spell,args:(announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:UpTo((kind:CountPlayers,args:(kind:OpponentsOf,args:(kind:AbilityController)))),filter:(kind:All,args:[(kind:Any),(kind:ControlledBy,args:(kind:OpponentsOf,args:(kind:AbilityController)))]),players:None,distribute:None,constraints:[DifferentControllers],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:Targets,args:(0))),changes:[(kind:SetController,args:(kind:AbilityController))],duration:Indefinite,evaluation:Snapshot))))])))");

pub static CARD: CardDef = CardDef { name: "Blatant Thievery", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
