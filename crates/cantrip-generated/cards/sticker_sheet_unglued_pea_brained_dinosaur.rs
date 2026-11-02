// Sticker sheet: Unglued, Pea-Brained, Dinosaur
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Unglued\")");
planned_item!("engine-no-stickers", "Name(\"Pea-Brained\")");
planned_item!("engine-no-stickers", "Name(\"Dinosaur\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:AddMana,args:(players:(kind:One,args:(kind:AbilityController)),mana:All([Amount(color:Colorless,amount:(kind:Constant,args:1)),Amount(color:Colorless,amount:(kind:Constant,args:1))]),restriction:Some(CastSpell((kind:CardType,args:Creature)))))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Turn(point:CombatBegins,players:(kind:One,args:(kind:AbilityController))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:Exactly((kind:Constant,args:1)),filter:(kind:All,args:[(kind:Not,args:(kind:CardType,args:Creature)),(kind:CardType,args:Artifact),(kind:ControlledBy,args:(kind:One,args:(kind:AbilityController)))]),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:Target,args:(slot:(0),index:0)))),changes:[(kind:SetBasePowerToughness,args:(power:(kind:Constant,args:4),toughness:(kind:Constant,args:4))),(kind:AddCardType,args:Artifact),(kind:AddCardType,args:Creature),(kind:GrantKeyword,args:(kind:Flying))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:4)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:8,toughness:6)");
