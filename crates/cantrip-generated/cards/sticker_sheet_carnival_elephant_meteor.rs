// Sticker sheet: Carnival, Elephant, Meteor
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Carnival\")");
planned_item!("engine-no-stickers", "Name(\"Elephant\")");
planned_item!("engine-no-stickers", "Name(\"Meteor\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Sacrifice(objects:(kind:One,args:(kind:Source)),choose:None)]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Draw,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:2)))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:Proliferate,object:None,other:None,amount:None,subtype:None))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:8,toughness:7)");
