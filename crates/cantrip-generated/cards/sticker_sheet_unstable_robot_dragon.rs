// Sticker sheet: Unstable, Robot, Dragon
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Unstable\")");
planned_item!("engine-no-stickers", "Name(\"Robot\")");
planned_item!("engine-no-stickers", "Name(\"Dragon\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(1)])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:SwitchPowerToughness)],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:EventSubject))),changes:[(kind:Pump,args:(power:(kind:Constant,args:5),toughness:(kind:Constant,args:5)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:3,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:2,toughness:7)");
