// Sticker sheet: Deep-Fried, Plague, Myr
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Deep-Fried\")");
planned_item!("engine-no-stickers", "Name(\"Plague\")");
planned_item!("engine-no-stickers", "Name(\"Myr\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Scry,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1)))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:Object(event:LeavesBattlefield,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:Either,args:[(kind:CardType,args:Artifact),(kind:CardType,args:Enchantment)]),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Optional,args:(decider:(kind:AbilityController),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Destroy,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),regeneration:true))))])))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:4,toughness:5)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:8,toughness:4)");
