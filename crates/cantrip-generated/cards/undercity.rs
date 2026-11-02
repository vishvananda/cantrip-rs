// Undercity
// status: planned
// requires: ability DungeonRoom
// requires: ability Unique
// requires: layout Dungeon
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_2_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_room_target_0)];
fn ability_2_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::PlusOne, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_3_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_5, asker.source, amount_5)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_4_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_4_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_4_room_target_0)];
fn ability_4_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_6 = { let q: &dyn Query = g; (5i64).max(0) as i32 }; let players_6 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.lose_life(player, amount_6); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_5_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_5_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_5_room_target_0)];
fn ability_5_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(object) = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::goad::goad(g, item, object); }
g.end_sentence();
 Ok(()) }
pub mod token_9 {
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
fn ability_6_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_8 = Vec::new();
let count_8_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_8_0 = g.card_id(&token_9::CARD);
for player in players_8 {
tokens_8.extend(std::iter::repeat((def_8_0,player)).take(count_8_0 as usize));
} let words_8 = Words::NONE;
let created_8 = g.create_tokens_worded(&tokens_8, false, None, words_8)?;
g.end_sentence();
 Ok(()) }
fn ability_7_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_10 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_10 { g.draw(player, amount_10); }
g.end_sentence();
 Ok(()) }
pub mod token_12 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Skeleton).name, token_words: token!(Skeleton).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Skeleton]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(4), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_8_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_11 = Vec::new();
let count_11_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_11_0 = g.card_id(&token_12::CARD);
for player in players_11 {
tokens_11.extend(std::iter::repeat((def_11_0,player)).take(count_11_0 as usize));
} let words_11 = g.words(asker.source);
let created_11 = g.create_tokens_worded(&tokens_11, false, None, words_11)?;
g.end_sentence();
 Ok(()) }
fn ability_9_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_13 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((10i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_13: Vec<PlayerId> = objects_13.iter().map(|o| g.state().obj(*o).owner).collect(); owners_13.dedup(); for owner in owners_13 { let cards: Vec<ObjectId> = objects_13.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let selected_14_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_14_candidates = { let q: &dyn Query = g; ((objects_13.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_14_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_14 = { cantrip_mechanics::choose::select(g, selected_14_player, asker.source, Purpose::PutOntoBattlefield, selected_14_candidates, cantrip_mechanics::select::Selection::of(selected_14_count, &[]))? };
let player_15 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_15 = ({ let q: &dyn Query = g; selected_14.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_15 = { let rows_15: Vec<(ObjectId, PlayerId, bool)> = objects_15.iter().map(|&o| (o, player_15, false)).collect(); let counters = [(CounterKind::PlusOne, { let q: &dyn Query = g; (3i64).max(0) as i16 })]; g.put_all_onto_battlefield_with_counters(&rows_15, &counters)? };
let objects_16 = { let q: &dyn Query = g; ((moved_15.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_16 = { let q: &dyn Query = g; Some(asker.source) }; let duration_16 = Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") });
let kind_16_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Hexproof])) };
if let Some(source) = source_16 { cantrip_api::instruction(g, |g| { for &object in &objects_16 { g.add_effect(source, object, kind_16_0_0, duration_16);  } }); }
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("ability-unique" => statics: game_rule(GameRule::VentureOnlyByName));
planned_item!("ability-dungeon-room" => rooms: Room { name: "Secret Entrance", effect: spell(&[], ability_1_room), leads_to: &["Forge", "Lost Well"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Forge", effect: spell(ABILITY_2_ROOM_TARGETS, ability_2_room), leads_to: &["Trap!", "Arena"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Lost Well", effect: spell(&[], ability_3_room), leads_to: &["Arena", "Stash"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Trap!", effect: spell(ABILITY_4_ROOM_TARGETS, ability_4_room), leads_to: &["Archives"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Arena", effect: spell(ABILITY_5_ROOM_TARGETS, ability_5_room), leads_to: &["Archives", "Catacombs"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Stash", effect: spell(&[], ability_6_room), leads_to: &["Catacombs"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Archives", effect: spell(&[], ability_7_room), leads_to: &["Throne of the Dead Three"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Catacombs", effect: spell(&[], ability_8_room), leads_to: &["Throne of the Dead Three"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Throne of the Dead Three", effect: spell(&[], ability_9_room), leads_to: &[] });
planned_item!("layout-dungeon" => dungeon: &[1, 2]);
planned_item!("layout-dungeon" => dungeon: &[3, 4]);
planned_item!("layout-dungeon" => dungeon: &[4, 5]);
planned_item!("layout-dungeon" => dungeon: &[6]);
planned_item!("layout-dungeon" => dungeon: &[6, 7]);
planned_item!("layout-dungeon" => dungeon: &[7]);
planned_item!("layout-dungeon" => dungeon: &[8]);
planned_item!("layout-dungeon" => dungeon: &[8]);
planned_item!("layout-dungeon" => dungeon: &[]);

pub static CARD: CardDef = CardDef { name: "Undercity", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[Subtype::Undercity]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_9::CARD,&token_12::CARD];
