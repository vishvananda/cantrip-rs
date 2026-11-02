// Sticker sheet: Happy, Dead, Squirrel
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Happy\")");
planned_item!("engine-no-stickers", "Name(\"Dead\")");
planned_item!("engine-no-stickers", "Name(\"Squirrel\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:AddMana,args:(players:(kind:One,args:(kind:AbilityController)),mana:All([Amount(color:Colorless,amount:(kind:Constant,args:1)),Amount(color:Colorless,amount:(kind:Constant,args:1))]),restriction:Some(CastSpell((kind:Not,args:(kind:CardType,args:Creature))))))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Keyword,args:(kind:Infect))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:3,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:4,toughness:7)");
