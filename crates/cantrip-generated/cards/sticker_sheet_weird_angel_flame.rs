// Sticker sheet: Weird, Angel, Flame
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Weird\")");
planned_item!("engine-no-stickers", "Name(\"Angel\")");
planned_item!("engine-no-stickers", "Name(\"Flame\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Spell(event:Cast,players:(kind:One,args:(kind:AbilityController)),spells:(kind:TargetsAny,args:(kind:Objects,args:(kind:One,args:(kind:Source))))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:Source))),kind:PowerToughness(1,1),amount:(kind:Constant,args:2),selection:None)]))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Keyword,args:(kind:Protection,args:Objects(kind:Any,filter:(kind:Property,args:(property:ManaValue,comparison:SameParity,value:(kind:Constant,args:0))))))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:7,toughness:8)");
