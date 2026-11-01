// Dungeon of the Mad Mage
// status: planned
// requires: ability DungeonRoom
// requires: layout Dungeon
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn ability_1_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_1, asker.source, amount_1)?;
g.end_sentence();
 Ok(()) }
pub mod token_3 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_2 = Vec::new();
let count_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_2_0 = g.card_id(&token_3::CARD);
for player in players_2 {
tokens_2.extend(std::iter::repeat((def_2_0,player)).take(count_2_0 as usize));
} let words_2 = Words::NONE;
let created_2 = g.create_tokens_worded(&tokens_2, false, None, words_2)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_3_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_room_target_0)];
fn ability_3_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_4 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::CantAttack };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_4_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_5, asker.source, amount_5)?;
g.end_sentence();
 Ok(()) }
fn ability_5_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_6 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_6 = g.exile_all(&objects_6);
g.end_sentence();
let caster_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_7: Vec<ObjectId> = { let q: &dyn Query = g; (moved_6.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_7: u32 = 4294967295; let mut played_7: Vec<ObjectId> = Vec::new();
while left_7 > 0 {
let mut open_7: Vec<ObjectId> = Vec::new(); for &c in &pool_7 { let ways = g.cast_ways_during_resolution_by(c, caster_7); if ways.iter().any(|w| true) || g.could_play_land_during_resolution(c) { open_7.push(c); } }
let picked_7 = if open_7.is_empty() { None } else if open_7.len() > 1 || (true && (!false || pool_7.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_7, asker.source, Purpose::Optional, pool_7.clone(), open_7.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_7, open_7[0], Purpose::Optional)? { Some(open_7[0]) } else { None };
let Some(card_7) = picked_7 else { break; };
let ways_7: Vec<CastWay> = g.cast_ways_during_resolution_by(card_7, caster_7).into_iter().filter(|w| true).collect();
let done_7 = if ways_7.is_empty() { g.play_land_during_resolution(card_7)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_7, card_7, ways_7)?.map(|w| w.way); g.cast_during_resolution_as(card_7, CastTerms { instead: None, way, caster: Some(caster_7), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_7 { left_7 -= 1; played_7.push(g.state().obj(card_7).became.unwrap_or(card_7)); } else if 4294967295i64 == 1 { break; } if done_7 || !(true && true) { pool_7.retain(|&c| c != card_7); }
}
g.end_sentence();
 Ok(()) }
pub mod token_9 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Skeleton).name, token_words: token!(Skeleton).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Skeleton]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_6_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_8 = Vec::new();
let count_8_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_8_0 = g.card_id(&token_9::CARD);
for player in players_8 {
tokens_8.extend(std::iter::repeat((def_8_0,player)).take(count_8_0 as usize));
} let words_8 = g.words(asker.source);
let created_8 = g.create_tokens_worded(&tokens_8, false, None, words_8)?;
g.end_sentence();
 Ok(()) }
fn ability_7_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_10 = { let q: &dyn Query = g; (3i64).max(0) as usize }; let player_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_10, asker.source, amount_10)?;
g.end_sentence();
 Ok(()) }
fn ability_8_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_11 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_11: i64 = 0; let mut drawn_11: Vec<ObjectId> = Vec::new(); for player in players_11 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_11); done_11 += g.history().drawn(player) as i64 - before; drawn_11.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
g.end_sentence();
let objects_12 = { let q: &dyn Query = g; drawn_11.clone() }; let mut owners_12: Vec<PlayerId> = objects_12.iter().map(|o| g.state().obj(*o).owner).collect(); owners_12.dedup(); for owner in owners_12 { let cards: Vec<ObjectId> = objects_12.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let caster_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_13: Vec<ObjectId> = { let q: &dyn Query = g; (drawn_11.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_13: u32 = 1; let mut played_13: Vec<ObjectId> = Vec::new();
while left_13 > 0 {
let mut open_13: Vec<ObjectId> = Vec::new(); for &c in &pool_13 { let ways = g.cast_ways_during_resolution_by(c, caster_13); if ways.iter().any(|w| true) { open_13.push(c); } }
let picked_13 = if open_13.is_empty() { None } else if open_13.len() > 1 || (true && (!true || pool_13.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_13, asker.source, Purpose::CastFree, pool_13.clone(), open_13.clone(), 1, false)?.first().copied() } else if !false || cantrip_mechanics::effects::ask_may(g, caster_13, open_13[0], Purpose::CastFree)? { Some(open_13[0]) } else { None };
let Some(card_13) = picked_13 else { break; };
let ways_13: Vec<CastWay> = g.cast_ways_during_resolution_by(card_13, caster_13).into_iter().filter(|w| true).collect();
let done_13 = if ways_13.is_empty() { g.play_land_during_resolution(card_13)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_13, card_13, ways_13)?.map(|w| w.way); g.cast_during_resolution_as(card_13, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_13), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_13 { left_13 -= 1; played_13.push(g.state().obj(card_13).became.unwrap_or(card_13)); } else if 1i64 == 1 { break; } if done_13 || !(false && true) { pool_13.retain(|&c| c != card_13); }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("ability-dungeon-room" => rooms: Room { name: "Yawning Portal", effect: spell(&[], ability_0_room), leads_to: &["Dungeon Level"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Dungeon Level", effect: spell(&[], ability_1_room), leads_to: &["Goblin Bazaar", "Twisted Caverns"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Goblin Bazaar", effect: spell(&[], ability_2_room), leads_to: &["Lost Level"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Twisted Caverns", effect: spell(ABILITY_3_ROOM_TARGETS, ability_3_room), leads_to: &["Lost Level"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Lost Level", effect: spell(&[], ability_4_room), leads_to: &["Runestone Caverns", "Muiral's Graveyard"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Runestone Caverns", effect: spell(&[], ability_5_room), leads_to: &["Deep Mines"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Muiral's Graveyard", effect: spell(&[], ability_6_room), leads_to: &["Deep Mines"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Deep Mines", effect: spell(&[], ability_7_room), leads_to: &["Mad Wizard's Lair"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Mad Wizard's Lair", effect: spell(&[], ability_8_room), leads_to: &[] });
planned_item!("layout-dungeon" => dungeon: &[&[1], &[2, 3], &[4], &[4], &[5, 6], &[7], &[7], &[8], &[]]);

pub static CARD: CardDef = CardDef { name: "Dungeon of the Mad Mage", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_3::CARD,&token_9::CARD];
