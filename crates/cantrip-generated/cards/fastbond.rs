// Fastbond
// status: planned
// requires: the first land played, where the card says it wasn't
// You may play any number of lands on each of your turns.
// Whenever you play a land, if it wasn't the first land you played this turn, this enchantment deals 1 damage to you.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("the-first-land-played-where-the-card-says-it-wasn-t", "(kind:Triggered,args:(event:Player(event:PlaysLand,players:(kind:One,args:(kind:AbilityController)),objects:Some((kind:CardType,args:Land))),intervening_if:Some((kind:ObjectMatches,args:(object:(kind:EventSubject),filter:(kind:State,args:FirstLandPlayedThisTurn)))),limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:DealDamage,args:(source:(kind:Source),recipients:(kind:Players,args:(kind:One,args:(kind:AbilityController))),amount:(kind:Constant,args:1),preventable:true))))])))");

pub static CARD: CardDef = CardDef { name: "Fastbond", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::AnyNumberOfLandPlays], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
