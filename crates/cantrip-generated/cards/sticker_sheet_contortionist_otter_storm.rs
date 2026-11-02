// Sticker sheet: Contortionist, Otter, Storm
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Contortionist\")");
planned_item!("engine-no-stickers", "Name(\"Otter\")");
planned_item!("engine-no-stickers", "Name(\"Storm\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:CardType,args:Creature),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Target,args:(slot:(0),index:0)))),changes:[(kind:GrantKeyword,args:(kind:Haste))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Keyword,args:(kind:Deathtouch)),(kind:Keyword,args:(kind:Lifelink))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:5,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:3,toughness:5)");
