// Sticker sheet: Jetpack, Death, Seltzer
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Jetpack\")");
planned_item!("engine-no-stickers", "Name(\"Death\")");
planned_item!("engine-no-stickers", "Name(\"Seltzer\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:Trample))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(3)])]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:Monstrosity,object:None,other:None,amount:Some((kind:Constant,args:3)),subtype:None))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:2,toughness:7)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:6,toughness:5)");
