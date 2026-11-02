// Sticker sheet: Notorious, Sliver, War
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Notorious\")");
planned_item!("engine-no-stickers", "Name(\"Sliver\")");
planned_item!("engine-no-stickers", "Name(\"War\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(5)])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:Battlefield,args:(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))]))),changes:[(kind:Pump,args:(power:(kind:Constant,args:1),toughness:(kind:Constant,args:1)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Keyword,args:(kind:Protection,args:Objects(kind:Any,filter:(kind:All,args:[(kind:CardType,args:Creature),(kind:Property,args:(property:CreatureTypeCount,comparison:GreaterOrEqual,value:(kind:Constant,args:2)))]))))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:3,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:9,toughness:6)");
