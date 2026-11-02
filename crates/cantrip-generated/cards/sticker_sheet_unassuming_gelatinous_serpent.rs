// Sticker sheet: Unassuming, Gelatinous, Serpent
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Unassuming\")");
planned_item!("engine-no-stickers", "Name(\"Gelatinous\")");
planned_item!("engine-no-stickers", "Name(\"Serpent\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Triggered,args:(event:ZoneChange(objects:(kind:SameObject,args:(kind:Source)),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Card,zones:[Graveyard])),count:Exactly((kind:Constant,args:1)),filter:(kind:All,args:[(kind:All,args:[(kind:Not,args:(kind:CardType,args:Creature)),(kind:Not,args:(kind:CardType,args:Land))]),(kind:InZone,args:Players(zone:Graveyard,players:(kind:One,args:(kind:AbilityController))))]),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Move,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),destination:OwnersHand,entry:(tapped:false,counters:[],states:[]),face_down:false))))])))])");
planned_item!("engine-no-stickers", "Abilities(tickets:4,abilities:[(kind:Triggered,args:(event:Damage((source_kind:Permanent,sources:(kind:SameObject,args:(kind:Source)),recipients:Players((kind:All)),kind:Any,grouping:SourceRecipient,subject:Source)),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Mill,args:(players:(kind:One,args:(kind:EventSubject)),amount:(kind:Multiply,args:((kind:Constant,args:2),(kind:EventAmount)))))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:2,toughness:3)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:7,toughness:2)");
