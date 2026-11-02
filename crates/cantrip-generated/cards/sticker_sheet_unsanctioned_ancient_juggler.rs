// Sticker sheet: Unsanctioned, Ancient, Juggler
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Unsanctioned\")");
planned_item!("engine-no-stickers", "Name(\"Ancient\")");
planned_item!("engine-no-stickers", "Name(\"Juggler\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:Bolster,object:None,other:None,amount:Some((kind:Constant,args:1)),subtype:None))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Keyword,args:(kind:Indestructible))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:3,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:5,toughness:4)");
