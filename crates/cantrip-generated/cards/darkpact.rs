// Darkpact
// status: planned
// requires: ante zone
// requires: card outside ordinary play
// Remove this card from your deck before playing if you're not playing for ante.
// You own target card in the ante. Exchange that card with the top card of your library.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("card-outside-ordinary-play" => deck_rules: DeckRule::OnlyWhenPlayingForAnte);
planned_item!("ante-zone", "(kind:Spell,args:(announcement:(targets:[(slot:(0),kind:Object((kind:Card,zones:[Ante])),count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Exchange,args:(kind:Ante,first:(kind:Object,args:(kind:Target,args:(slot:(0),index:0))),second:(kind:Objects,args:(kind:Take,args:(objects:(kind:Zone,args:(scope:Players(zone:Library,players:(kind:One,args:(kind:AbilityController))),kind:Card,filter:(kind:Any))),count:(kind:Constant,args:1),from:Top))),until:None))))])))");

pub static CARD: CardDef = CardDef { name: "Darkpact", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 3, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
