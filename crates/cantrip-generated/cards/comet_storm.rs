// Comet Storm
// status: planned
// requires: a spell's target count of times kicked, beside an X or added to
// requires: engine: X in a mana cost beside a kicker
// Multikicker {1} (You may pay an additional {1} any number of times as you cast this spell.)
// Choose any target, then choose another target for each time this spell was kicked. Comet Storm deals X damage to each of them.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("a-spell-s-target-count-of-times-kicked-beside-an-x-or-added-to", "(kind:Spell,args:(announcement:(targets:[(slot:(0),kind:DamageRecipient,count:Exactly((kind:Add,args:((kind:Constant,args:1),(kind:TimesKicked,args:(kind:Source))))),filter:(kind:Any),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:DealDamage,args:(source:(kind:Source),recipients:(kind:Targets,args:(0)),amount:(kind:AnnouncedX),preventable:true))))])))");
planned_item!("engine-x-in-a-mana-cost-beside-a-kicker", "multikicker(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })");

pub static CARD: CardDef = CardDef { name: "Comet Storm", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: multikicker(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
