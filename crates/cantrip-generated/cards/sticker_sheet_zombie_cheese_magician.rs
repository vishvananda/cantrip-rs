// Sticker sheet: Zombie, Cheese, Magician
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Zombie\")");
planned_item!("engine-no-stickers", "Name(\"Cheese\")");
planned_item!("engine-no-stickers", "Name(\"Magician\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:FirstStrike))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Damage((source_kind:Permanent,sources:(kind:SameObject,args:(kind:Source)),recipients:Players((kind:All)),kind:Combat,grouping:SourceRecipient,subject:Source)),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Draw,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:EventAmount)))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:3,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:6,toughness:2)");
