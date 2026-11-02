// Sticker sheet: Squid, Fire, Knight
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Squid\")");
planned_item!("engine-no-stickers", "Name(\"Fire\")");
planned_item!("engine-no-stickers", "Name(\"Knight\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:Player,count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:Some((kind:All)),distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(1),operation:(kind:CreateReplacement,args:(event:Player(event:RollDice,players:(kind:One,args:(kind:Target,args:(slot:(0),index:0)))),limit:NextOccurrence,duration:EndOfTurn,body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:ModifyEvent,args:(change:RollExtraDie(ChosenBy((kind:AbilityController)))))))])))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Keyword,args:(kind:Protection,args:Objects(kind:Any,filter:(kind:Property,args:(property:ManaValue,comparison:SameParity,value:(kind:Constant,args:1))))))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:6,toughness:6)");
