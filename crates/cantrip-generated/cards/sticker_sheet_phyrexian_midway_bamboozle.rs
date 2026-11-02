// Sticker sheet: Phyrexian, Midway, Bamboozle
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Phyrexian\")");
planned_item!("engine-no-stickers", "Name(\"Midway\")");
planned_item!("engine-no-stickers", "Name(\"Bamboozle\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:GetTickets,object:None,other:None,amount:Some((kind:Constant,args:1)),subtype:None))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Mechanic,args:(mechanic:Undying,cost:None,amount:None,filter:None))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:6,toughness:9)");
