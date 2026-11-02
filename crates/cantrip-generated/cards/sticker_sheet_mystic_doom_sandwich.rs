// Sticker sheet: Mystic, Doom, Sandwich
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Mystic\")");
planned_item!("engine-no-stickers", "Name(\"Doom\")");
planned_item!("engine-no-stickers", "Name(\"Sandwich\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:Lifelink))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Static,args:(kind:Ordinary,affected:Objects((kind:One,args:(kind:Source))),changes:[(kind:Rule,args:RequireCombat(action:BeBlocked((kind:Any)),amount:AtLeast((kind:Constant,args:1))))])),(kind:Triggered,args:(event:Object(event:BecomesBlocked,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateContinuousEffect,args:(source:(kind:Source),affected:Objects((kind:One,args:(kind:EventSubject))),changes:[(kind:Pump,args:(power:(kind:CountObjects,args:(kind:Battlefield,args:(kind:All,args:[(kind:CardType,args:Creature),(kind:BlockingObject,args:(kind:EventSubject))]))),toughness:(kind:CountObjects,args:(kind:Battlefield,args:(kind:All,args:[(kind:CardType,args:Creature),(kind:BlockingObject,args:(kind:EventSubject))])))))],duration:EndOfTurn,evaluation:Snapshot))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:1,toughness:4)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:6,toughness:8)");
