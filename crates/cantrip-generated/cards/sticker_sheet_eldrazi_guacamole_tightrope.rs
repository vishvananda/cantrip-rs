// Sticker sheet: Eldrazi, Guacamole, Tightrope
// status: planned
// requires: engine: no stickers
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-stickers", "Name(\"Eldrazi\")");
planned_item!("engine-no-stickers", "Name(\"Guacamole\")");
planned_item!("engine-no-stickers", "Name(\"Tightrope\")");
planned_item!("engine-no-stickers", "Abilities(tickets:2,abilities:[(kind:Keyword,args:(kind:Haste))])");
planned_item!("engine-no-stickers", "Abilities(tickets:5,abilities:[(kind:Scoped,args:(zones:[Graveyard],condition:None,ability:(kind:Static,args:(kind:Ordinary,affected:Players((kind:One,args:(kind:AbilityController))),changes:[(kind:PlayerRule,args:PlayPermissionWith(permission:PlayPermission(cards:(kind:One,args:(kind:Source)),from:Graveyard,action:CastSpell,filter:(kind:Any)),terms:(limit:None,during:None,cost:Additional((kind:Cost,args:(parts:[Life((kind:Constant,args:2))]))),spend_as:None,into_exile:false,condition:None,effects:[],flash:false,may_look:false,then:None,face_down:None)))]))))])");
planned_item!("engine-no-stickers", "PowerToughness(tickets:2,power:1,toughness:4)");
planned_item!("engine-no-stickers", "PowerToughness(tickets:3,power:5,toughness:3)");
