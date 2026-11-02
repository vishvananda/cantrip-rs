// Sticker sheet: Werewolf, Lightning, Mage
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Werewolf\")");
planned_item!("engine-no-stickers", "Name(\"Lightning\")");
planned_item!("engine-no-stickers", "Name(\"Mage\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Player(event:EntersUnderControl,players:(kind:One,args:(kind:AbilityController)),objects:Some((kind:CardType,args:Land))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:Source))),kind:PowerToughness(1,1),amount:(kind:Constant,args:1),selection:None)]))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Pair(event:Blocks,objects:(kind:CardType,args:Creature),others:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:BlockPair,args:Other))),changes:[(kind:Pump,args:(power:(kind:Constant,args:-4),toughness:(kind:Constant,args:-4)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:4,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:3,toughness:5)");
