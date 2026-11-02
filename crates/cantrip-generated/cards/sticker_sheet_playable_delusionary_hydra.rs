// Sticker sheet: Playable, Delusionary, Hydra
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Playable\")");
planned_item!("engine-no-stickers", "Name(\"Delusionary\")");
planned_item!("engine-no-stickers", "Name(\"Hydra\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Draw,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1))))),(kind:Effect,args:(result:(1),operation:(kind:Discard,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1),random:false))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:GainLife,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:3))))),(kind:Effect,args:(result:(1),operation:(kind:Draw,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1)))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:1,toughness:5)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:4,toughness:4)");
