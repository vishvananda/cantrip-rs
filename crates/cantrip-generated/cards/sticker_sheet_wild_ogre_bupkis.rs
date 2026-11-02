// Sticker sheet: Wild, Ogre, Bupkis
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Wild\")");
planned_item!("engine-no-stickers", "Name(\"Ogre\")");
planned_item!("engine-no-stickers", "Name(\"Bupkis\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:One,args:(kind:EventSubject))),kind:PowerToughness(1,1),amount:(kind:Constant,args:1),selection:None)]))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Conditional,args:(condition:(kind:Compare,args:(left:(kind:CountObjects,args:(kind:Battlefield,args:(kind:All,args:[(kind:CardType,args:Artifact),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))]))),comparison:GreaterOrEqual,right:(kind:Constant,args:3))),then:[(kind:Static,args:(kind:Ordinary,affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:GrantKeyword,args:(kind:Protection,args:Objects(kind:Permanent,filter:(kind:Not,args:(kind:CardType,args:Creature)))))]))],otherwise:[]))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:5,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:7,toughness:4)");
