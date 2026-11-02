// Sticker sheet: Vampire, Champion, Fury
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Vampire\")");
planned_item!("engine-no-stickers", "Name(\"Champion\")");
planned_item!("engine-no-stickers", "Name(\"Fury\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Conditional,args:(condition:(kind:Compare,args:(left:(kind:CountObjects,args:(kind:Zone,args:(scope:Players(zone:Hand,players:(kind:One,args:(kind:AbilityController))),kind:Card,filter:(kind:Any)))),comparison:Equal,right:(kind:Constant,args:0))),then:[(kind:Static,args:(kind:Ordinary,affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:Pump,args:(power:(kind:Constant,args:3),toughness:(kind:Constant,args:3)))]))],otherwise:[]))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(2)]),Sacrifice(objects:(kind:One,args:(kind:Source)),choose:None)]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:AnyNumber,filter:(kind:CardType,args:Creature),players:None,distribute:Some((kind:ObjectProperty,args:(object:(kind:Query,args:(objects:(kind:CostObjects,args:(payment:Activation,component:[1],view:Inputs,phase:Before)),index:0)),property:Power,information:CurrentOrLastKnown))),constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:DealDamage,args:(source:(kind:Query,args:(objects:(kind:CostObjects,args:(payment:Activation,component:[1],view:Inputs,phase:Before)),index:0)),recipients:(kind:Objects,args:(kind:Targets,args:(0))),amount:(kind:DistributedShare,args:(0)),preventable:true))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:1,toughness:5)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:6,toughness:3)");
