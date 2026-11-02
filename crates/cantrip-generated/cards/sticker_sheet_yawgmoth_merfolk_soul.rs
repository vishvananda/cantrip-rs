// Sticker sheet: Yawgmoth, Merfolk, Soul
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Yawgmoth\")");
planned_item!("engine-no-stickers", "Name(\"Merfolk\")");
planned_item!("engine-no-stickers", "Name(\"Soul\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:Object(event:LeavesBattlefield,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Player,count:Exactly((kind:Constant,args:1)),filter:(kind:Any),players:Some((kind:All)),distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Discard,args:(players:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),amount:(kind:Constant,args:1),random:false))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:5,abilities:[(kind:Triggered,args:(event:Object(event:LeavesBattlefield,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateTokens,args:(players:(kind:One,args:(kind:AbilityController)),tokens:[(amount:(kind:Constant,args:5),words:CreatingEffect,recipe:Defined((name:FromSubtypes,colors:[White],chosen_color:false,chosen_creature_type:false,supertypes:[],card_types:[Artifact,Creature],subtypes:[Clown,Robot],power_toughness:Some(((kind:Constant,args:1),(kind:Constant,args:1))),abilities:[],abilities_of:None)))],entry:(tapped:false,counters:[],states:[])))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:3,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:4,power:6,toughness:5)");
