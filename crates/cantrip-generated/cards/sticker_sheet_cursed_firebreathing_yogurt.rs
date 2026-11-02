// Sticker sheet: Cursed, Firebreathing, Yogurt
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Cursed\")");
planned_item!("engine-no-stickers", "Name(\"Firebreathing\")");
planned_item!("engine-no-stickers", "Name(\"Yogurt\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:Prowess)),(kind:Keyword,args:(kind:Prowess))])");
planned_item!("engine-no-stickers", "Abilities(tickets:5,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(2)]),TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[(slot:(0),kind:DamageRecipient,count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:DealDamage,args:(source:(kind:Source),recipients:(kind:Targets,args:(0)),amount:(kind:Constant,args:2),preventable:true))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:4,toughness:8)");
