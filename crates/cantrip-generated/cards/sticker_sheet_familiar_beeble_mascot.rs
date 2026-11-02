// Sticker sheet: Familiar, Beeble, Mascot
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Familiar\")");
planned_item!("engine-no-stickers", "Name(\"Beeble\")");
planned_item!("engine-no-stickers", "Name(\"Mascot\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Tap,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),tapped:false))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Player(event:EntersUnderControl,players:(kind:One,args:(kind:AbilityController)),objects:Some((kind:CardType,args:Creature))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:Battlefield,args:(kind:All,args:[(kind:CardType,args:Creature),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))]))),changes:[(kind:Pump,args:(power:(kind:Constant,args:1),toughness:(kind:Constant,args:1)))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:5,toughness:3)");
