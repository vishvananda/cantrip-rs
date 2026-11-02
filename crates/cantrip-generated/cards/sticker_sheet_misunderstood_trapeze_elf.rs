// Sticker sheet: Misunderstood, Trapeze, Elf
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Misunderstood\")");
planned_item!("engine-no-stickers", "Name(\"Trapeze\")");
planned_item!("engine-no-stickers", "Name(\"Elf\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Spell(event:Cast,players:(kind:One,args:(kind:AbilityController)),spells:(kind:Any)),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:BindValue,args:(value:(kind:ObjectProperty,args:(object:(kind:EventSubject),property:GenericManaInCost,information:CurrentOrLastKnown)))))),(kind:Effect,args:(result:(1),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:Pump,args:(power:(kind:BoundValue,args:(0)),toughness:(kind:BoundValue,args:(0))))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Keyword,args:(kind:Hexproof))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:6,toughness:6)");
