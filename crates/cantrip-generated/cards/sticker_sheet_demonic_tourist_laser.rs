// Sticker sheet: Demonic, Tourist, Laser
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Demonic\")");
planned_item!("engine-no-stickers", "Name(\"Tourist\")");
planned_item!("engine-no-stickers", "Name(\"Laser\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Mechanic,args:(mechanic:Outlast,cost:Some((parts:[Mana([Generic(1)])])),amount:None,filter:None))])");
planned_item!("engine-no-stickers", "Abilities(tickets:3,abilities:[(kind:Triggered,args:(event:ZoneChange(objects:(kind:SameObject,args:(kind:Source)),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Keyword,args:(actor:(kind:One,args:(kind:AbilityController)),action:GetTickets,object:None,other:None,amount:Some((kind:Constant,args:7)),subtype:None))))])))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:1,toughness:4)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:5,power:9,toughness:6)");
