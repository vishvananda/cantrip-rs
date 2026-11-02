// Sticker sheet: Goblin, Coward, Parade
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Goblin\")");
planned_item!("engine-no-stickers", "Name(\"Coward\")");
planned_item!("engine-no-stickers", "Name(\"Parade\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Mechanic,args:(mechanic:Mentor,cost:None,amount:None,filter:None))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:Object(event:LeavesBattlefield,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:All,args:[(kind:CardType,args:Creature),(kind:Property,args:(property:Power,comparison:GreaterOrEqual,value:(kind:Constant,args:4)))]),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Optional,args:(decider:(kind:AbilityController),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Destroy,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),regeneration:true))))])))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:8,toughness:4)");
