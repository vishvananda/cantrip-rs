// Sticker sheet: Snazzy, Aether, Homunculus
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Snazzy\")");
planned_item!("engine-no-stickers", "Name(\"Aether\")");
planned_item!("engine-no-stickers", "Name(\"Homunculus\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(1)])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:CardType,args:Creature),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Target,args:(slot:(0),index:0)))),changes:[(kind:CreatureTypes,args:(mode:Set,types:All))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:Either([Spell(event:Cast,players:(kind:One,args:(kind:AbilityController)),spells:(kind:Either,args:[(kind:CardType,args:Instant),(kind:CardType,args:Sorcery)])),Spell(event:Copy,players:(kind:One,args:(kind:AbilityController)),spells:(kind:Either,args:[(kind:CardType,args:Instant),(kind:CardType,args:Sorcery)]))]),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Draw,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1)))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:4)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:8,toughness:7)");
