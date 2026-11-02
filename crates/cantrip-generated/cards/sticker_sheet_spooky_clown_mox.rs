// Sticker sheet: Spooky, Clown, Mox
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Spooky\")");
planned_item!("engine-no-stickers", "Name(\"Clown\")");
planned_item!("engine-no-stickers", "Name(\"Mox\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:Vigilance))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(1)]),TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:CardType,args:Creature),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Tap,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),tapped:true))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:1,toughness:5)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:5,toughness:4)");
