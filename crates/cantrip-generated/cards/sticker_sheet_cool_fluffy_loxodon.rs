// Sticker sheet: Cool, Fluffy, Loxodon
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Cool\")");
planned_item!("engine-no-stickers", "Name(\"Fluffy\")");
planned_item!("engine-no-stickers", "Name(\"Loxodon\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:LeavesBattlefield,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Draw,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:1)))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:5,abilities:[(kind:Triggered,args:(event:Player(event:EntersUnderControl,players:(kind:One,args:(kind:AbilityController)),objects:Some((kind:CardType,args:Creature))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:SetBasePowerToughness,args:(power:(kind:Constant,args:13),toughness:(kind:Constant,args:13))),(kind:CreatureTypes,args:(mode:Add,types:Listed([Eldrazi]))),(kind:AddCardType,args:Creature)],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:2)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:5,toughness:6)");
