// Sticker sheet: Trendy, Circus, Pirate
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Trendy\")");
planned_item!("engine-no-stickers", "Name(\"Circus\")");
planned_item!("engine-no-stickers", "Name(\"Pirate\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:Deathtouch))])");
planned_item!("engine-no-stickers", "Abilities(tickets:5,abilities:[(kind:Triggered,args:(event:Damage((source_kind:Permanent,sources:(kind:SameObject,args:(kind:Source)),recipients:Players((kind:All)),kind:Combat,grouping:SourceRecipient,subject:Source)),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateTokens,args:(players:(kind:One,args:(kind:AbilityController)),tokens:[(amount:(kind:EventAmount),words:CreatingEffect,recipe:Defined((name:FromSubtypes,colors:[Green],chosen_color:false,chosen_creature_type:false,supertypes:[],card_types:[Creature],subtypes:[Squirrel],power_toughness:Some(((kind:Constant,args:1),(kind:Constant,args:1))),abilities:[],abilities_of:None)))],entry:(tapped:false,counters:[],states:[])))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:5,toughness:1)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:3,toughness:6)");
