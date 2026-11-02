// Sticker sheet: Unique, Charmed, Pants
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Unique\")");
planned_item!("engine-no-stickers", "Name(\"Charmed\")");
planned_item!("engine-no-stickers", "Name(\"Pants\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:AddMana,args:(players:(kind:One,args:(kind:AbilityController)),mana:ChooseOne([Amount(color:White,amount:(kind:Constant,args:1)),Amount(color:Blue,amount:(kind:Constant,args:1)),Amount(color:Black,amount:(kind:Constant,args:1)),Amount(color:Red,amount:(kind:Constant,args:1)),Amount(color:Green,amount:(kind:Constant,args:1))]),restriction:None))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:Some((kind:ObjectMatches,args:(object:(kind:EventSubject),filter:(kind:Not,args:(kind:Subtype,args:Brushwagg))))),limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:EventSubject))),changes:[(kind:Pump,args:(power:(kind:Add,args:((kind:Add,args:((kind:ObjectProperty,args:(object:(kind:EventSubject),property:SupertypeCount,information:CurrentOrLastKnown)),(kind:CountDistinct,args:(objects:(kind:One,args:(kind:EventSubject)),property:CardTypes)))),(kind:ObjectProperty,args:(object:(kind:EventSubject),property:SubtypeCount,information:CurrentOrLastKnown)))),toughness:(kind:Constant,args:0)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:5,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:4,toughness:8)");
