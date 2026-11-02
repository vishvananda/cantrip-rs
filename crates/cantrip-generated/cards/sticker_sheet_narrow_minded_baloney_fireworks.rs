// Sticker sheet: Narrow-Minded, Baloney, Fireworks
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Narrow-Minded\")");
planned_item!("engine-no-stickers", "Name(\"Baloney\")");
planned_item!("engine-no-stickers", "Name(\"Fireworks\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:GainLife,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:2)))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Keyword,args:(kind:Vigilance)),(kind:Keyword,args:(kind:Reach))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:4)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:7,toughness:7)");
