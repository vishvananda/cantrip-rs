// Archival Whorl
// status: planned
// requires: a gift of another kind
// Gift a Rhystic Study
// Shuffle your hand and graveyard into your library, then draw seven cards. If the gift wasn't promised, each other player also shuffles their hand and graveyard into their library, then draws seven cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_stated_gift_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-reference-gifted-player", Conjure, "actor" = "{\"args\":{\"kind\":\"GiftedPlayer\"},\"kind\":\"One\"}", "card" = "{\"Named\":\"Rhystic Study\"}", "count" = { let q: &dyn Query = g; 1i64 }, "entry" = "[{\"args\":[{\"args\":{\"family\":\"EnterFlag\",\"form\":\"EntersNormally\",\"operands\":[]},\"kind\":\"Term\"}],\"kind\":\"List\"}]", "owner" = "()", "spot" = "()", "zone" = "\"Battlefield\"");
g.end_sentence();
 Ok(()) }
fn ability_0_stated_promised(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_1 = g.put_into_library(&objects_1, LibraryPlace::Top);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
 Ok(()) }
fn ability_0_stated_unpromised(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let moved_4 = g.put_into_library(&objects_4, LibraryPlace::Top);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; { let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } };
let mut gathered_8: Vec<ObjectId> = Vec::new();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { gathered_8.extend({ let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } });
    } 
let objects_10 = ({ let q: &dyn Query = g; gathered_8.clone() }).into_iter().rev().collect::<Vec<ObjectId>>(); let moved_10 = g.put_into_library(&objects_10, LibraryPlace::Top);
g.end_sentence();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
let amount_12 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_12 = { let q: &dyn Query = g; { let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } };
for player in players_12 { g.draw(player, amount_12); }
g.end_sentence();
 Ok(()) }
planned_item!("a-gift-of-another-kind" => gift: Some(GiftDef { kind: Gift::Custom, give: ability_0_stated_gift_0 }));
planned_item!("a-gift-of-another-kind" => spell: cantrip_mechanics::gift::spell(Some(SpellBranch::Plain(&[], ability_0_stated_promised)), Some(SpellBranch::Plain(&[], ability_0_stated_unpromised))));

pub static CARD: CardDef = CardDef { name: "Archival Whorl", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
