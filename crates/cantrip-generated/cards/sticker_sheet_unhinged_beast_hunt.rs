// Sticker sheet: Unhinged, Beast, Hunt
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Unhinged\")");
planned_item!("engine-no-stickers", "Name(\"Beast\")");
planned_item!("engine-no-stickers", "Name(\"Hunt\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:GainLife,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1)))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Tap,args:(objects:(kind:Battlefield,args:(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:OpponentsOf,args:(kind:AbilityController))),(kind:Either,args:[(kind:Property,args:(property:Power,comparison:Equal,value:(kind:ObjectProperty,args:(object:(kind:Source),property:Power,information:CurrentOrLastKnown)))),(kind:Property,args:(property:Toughness,comparison:Equal,value:(kind:ObjectProperty,args:(object:(kind:Source),property:Toughness,information:CurrentOrLastKnown))))])])),tapped:true))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:2,toughness:6)");
