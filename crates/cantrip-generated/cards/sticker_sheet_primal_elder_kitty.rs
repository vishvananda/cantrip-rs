// Sticker sheet: Primal, Elder, Kitty
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Primal\")");
planned_item!("engine-no-stickers", "Name(\"Elder\")");
planned_item!("engine-no-stickers", "Name(\"Kitty\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(1)])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:Pump,args:(power:(kind:Constant,args:1),toughness:(kind:Constant,args:-1)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:ZoneChange(objects:(kind:SameObject,args:(kind:Source)),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:CardType,args:Creature),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:BindValue,args:(value:(kind:ObjectProperty,args:(object:(kind:EventSubject),property:Power,information:CurrentOrLastKnown)))))),(kind:Optional,args:(decider:(kind:AbilityController),body:(instructions:[(kind:Effect,args:(result:(1),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:Target,args:(slot:(0),index:0)))),kind:PowerToughness(1,1),amount:(kind:BoundValue,args:(0)),selection:None)]))))])))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:5,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:4,toughness:7)");
