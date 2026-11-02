// Sticker sheet: Trained, Blessed, Mind
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Trained\")");
planned_item!("engine-no-stickers", "Name(\"Blessed\")");
planned_item!("engine-no-stickers", "Name(\"Mind\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:Object((kind:Card,zones:[Graveyard])),count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Move,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),destination:Exile,entry:(tapped:false,counters:[],states:[]),face_down:false))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Conditional,args:(condition:(kind:Compare,args:(left:(kind:CountObjects,args:(kind:Zone,args:(scope:Players(zone:Graveyard,players:(kind:One,args:(kind:AbilityController))),kind:Card,filter:(kind:Any)))),comparison:GreaterOrEqual,right:(kind:Constant,args:7))),then:[(kind:Static,args:(kind:Ordinary,affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:Pump,args:(power:(kind:Constant,args:4),toughness:(kind:Constant,args:0))),(kind:GrantKeyword,args:(kind:Trample))]))],otherwise:[]))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:6,toughness:9)");
