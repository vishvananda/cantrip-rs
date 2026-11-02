// Sticker sheet: Geek, Lotus, Warrior
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Geek\")");
planned_item!("engine-no-stickers", "Name(\"Lotus\")");
planned_item!("engine-no-stickers", "Name(\"Warrior\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(2)])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:Pump,args:(power:(kind:Constant,args:2),toughness:(kind:Constant,args:0)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Player(event:EntersUnderControl,players:(kind:One,args:(kind:AbilityController)),objects:Some((kind:CardType,args:Creature))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Player,count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:Some((kind:All)),distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:DealDamage,args:(source:(kind:Source),recipients:(kind:Players,args:(kind:One,args:(kind:Target,args:(slot:(0),index:0)))),amount:(kind:Constant,args:2),preventable:true))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:3,toughness:6)");
