// Baldur's Gate Wilderness
// status: planned
// requires: ability DungeonRoom
// requires: engine: no commander
// requires: keyword action RollDie(4) with an amount
// requires: layout Dungeon
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_0 = cantrip_mechanics::library::searched_cards(g, owner_0); let can_0: Vec<ObjectId> = { let q: &dyn Query = g; shown_0.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_0 = cantrip_mechanics::library::search(g, searcher_0, owner_0, asker.source, SearchDestination::Hand, cantrip_mechanics::select::Selection::listed(rule_count_0, &shown_0, can_0))?;
let objects_1 = { let q: &dyn Query = g; found_0.clone() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_2 = { let q: &dyn Query = g; found_0.clone() }; let moved_2 = g.return_to_hand_revealed(&objects_2);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
pub mod token_5 {
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
fn ability_1_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_4 = Vec::new();
let count_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_4_0 = g.card_id(&token_5::CARD);
for player in players_4 {
tokens_4.extend(std::iter::repeat((def_4_0,player)).take(count_4_0 as usize));
} let words_4 = Words::NONE;
let created_4 = g.create_tokens_worded(&tokens_4, false, None, words_4)?;
g.end_sentence();
 Ok(()) }
pub mod token_7 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Knight).name, token_words: token!(Knight).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_6 = Vec::new();
let count_6_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_6_0 = g.card_id(&token_7::CARD);
for player in players_6 {
tokens_6.extend(std::iter::repeat((def_6_0,player)).take(count_6_0 as usize));
} let words_6 = g.words(asker.source);
let created_6 = g.create_tokens_worded(&tokens_6, false, None, words_6)?;
g.end_sentence();
 Ok(()) }
fn ability_3_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_8 = { let q: &dyn Query = g; (3i64).max(0) as usize }; let player_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_8, asker.source, amount_8)?;
g.end_sentence();
 Ok(()) }
fn ability_4_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_9, after_9, before_9, picked_9): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let objects_10 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() }; let players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_10: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_10 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; for player in players_10 { let eligible: Vec<ObjectId> = objects_10.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_10.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() < n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_10.clone(), &[]))? }; for object in taken { picks_10.push((player, object)); } }
let sacrificed_10 = g.sacrifice_each(&picks_10);
let paid_10 = !picks_10.is_empty();
(paid_10, { let q: &dyn Query = g; (sacrificed_10).clone() }, { let q: &dyn Query = g; (objects_10.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_9 as i64) != 0) } {
let amount_11 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_11 { g.draw(player, amount_11); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_5_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_12_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_12_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let selected_12_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_12 = { cantrip_mechanics::choose::select(g, selected_12_player, asker.source, Purpose::PutOntoBattlefield, selected_12_candidates, cantrip_mechanics::select::Selection::of(selected_12_count, &[]))? };
if !selected_12.is_empty() { 
let player_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_13 = ({ let q: &dyn Query = g; selected_12.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_13 = { let rows_13: Vec<(ObjectId, PlayerId, bool)> = objects_13.iter().map(|&o| (o, player_13, false)).collect(); let came_13 = g.put_all_onto_battlefield_as(&rows_13)?; came_13.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_14.extend(moved_13.clone());
hoisted_15.extend(selected_12.clone());
hoisted_16.extend({ let q: &dyn Query = g; objects_13.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
pub mod token_18 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Faerie Dragon).name, token_words: token!(Faerie Dragon).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(2)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_6_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_17 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_17 = Vec::new();
let count_17_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_17_0 = g.card_id(&token_18::CARD);
for player in players_17 {
tokens_17.extend(std::iter::repeat((def_17_0,player)).take(count_17_0 as usize));
} let words_17 = g.words(asker.source);
let created_17 = g.create_tokens_worded(&tokens_17, false, None, words_17)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_7_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::controlled_by_slot_opponent(q, id)) && (matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature)))) }
static ABILITY_7_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_7_room_target_0).between(0, TargetSpec::EACH_OPPONENT)];
fn ability_7_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
for bound_4294967276 in { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967276).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::goad::goad(g, item, object); }
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_8_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_8_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_8_room_target_0).between(0, 3)];
fn ability_8_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
cantrip_mechanics::counters_divided(g, item, CounterKind::PlusOne);
g.end_sentence();
 Ok(()) }
fn ability_9_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_22 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_22 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_22 { g.draw(player, amount_22); }
g.end_sentence();
 Ok(()) }
pub mod token_25 {
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
fn ability_10_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_23 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let player_23 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_23: i64 = planned_value!("keyword action RollDie(4) with an amount", 0i64 => cantrip_mechanics::dice::roll_several(g, player_23, 4, amount_23)?.total());
g.end_sentence();
let players_24 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_24 = Vec::new();
let count_24_0 = { let q: &dyn Query = g; (value_23).clamp(0, 4294967295) as u32 }; let def_24_0 = g.card_id(&token_25::CARD);
for player in players_24 {
tokens_24.extend(std::iter::repeat((def_24_0,player)).take(count_24_0 as usize));
} let words_24 = Words::NONE;
let created_24 = g.create_tokens_worded(&tokens_24, false, None, words_24)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn lasting_cost_27(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if (q.chars(spell).is(CardType::Instant) || q.chars(spell).is(CardType::Sorcery)) { CostDelta::less((3i64) as u8) } else { CostDelta::NONE } }
fn ability_11_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_26_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_26_0 { g.add_player_effect(asker.controller, source, EffectKind::SpellCostChange { card: item.ability.card, index: 0, player }, Duration::EndOfTurn); } }
g.end_sentence();
 Ok(()) }
fn ability_12_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_28 = { let q: &dyn Query = g; (5i64).max(0) as i32 }; let players_28 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_28 { g.lose_life(player, amount_28); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_13_room_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_13_ROOM_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_13_room_target_0).between(0, 2)];
fn ability_13_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_29 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_29 = g.return_to_hand(&objects_29);
g.end_sentence();
 Ok(()) }
fn ability_14_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_30 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let among_30 = { let q: &dyn Query = g; ({ let mut all: Vec<ObjectId> = q.state().battlefield().to_vec(); all.extend_from_slice(q.state().zone(Zone::STACK)); all.extend_from_slice(q.state().zone(Zone::COMMAND)); for p in q.state().seats() { for zone in [Zone::library(p), Zone::hand(p), Zone::graveyard(p), Zone::exile(p)] { all.extend_from_slice(q.state().zone(zone)); } } all }).into_iter().filter(|&id| (planned_value!("engine: no commander", false => q.is_commander(id)) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let chosen_30 = cantrip_mechanics::choose::select(g, player_30, asker.source, Purpose::CopyWhich, among_30, cantrip_mechanics::select::cards(cantrip_mechanics::select::exactly(1)))?.first().copied();
let values_30 = { let q: &dyn Query = g; chosen_30.map(|original: ObjectId| q.copiable_values(original).nonlegendary()) }; let count_30 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let created_30 = match values_30 { Some(values) => g.create_token_copies_tapped(values, player_30, count_30, false)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
pub mod token_32 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Skeleton).name, token_words: token!(Skeleton).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Skeleton]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(4), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_15_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_31 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_31 = Vec::new();
let count_31_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let def_31_0 = g.card_id(&token_32::CARD);
for player in players_31 {
tokens_31.extend(std::iter::repeat((def_31_0,player)).take(count_31_0 as usize));
} let words_31 = g.words(asker.source);
let created_31 = g.create_tokens_worded(&tokens_31, false, None, words_31)?;
g.end_sentence();
 Ok(()) }
pub mod emblem_33 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Trample); }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Baldur's Gate Wilderness emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_1, "Static: GrantKeyword."),static_ability(Layer::PtModify, ability_0_affects, ability_0_apply_0, "Static: Pump.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_16_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_33::CARD); }
g.end_sentence();
 Ok(()) }
fn ability_17_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_34 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_34: Vec<PlayerId> = objects_34.iter().map(|o| g.state().obj(*o).owner).collect(); owners_34.dedup(); for owner in owners_34 { let cards: Vec<ObjectId> = objects_34.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let objects_35 = { let q: &dyn Query = g; (objects_34.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let moved_35 = g.return_to_hand_revealed(&objects_35);
g.end_sentence();
let amount_36 = { let q: &dyn Query = g; ((objects_34.clone()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>()).min(i32::MAX.into()) as i32 }; let players_36 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_36 { g.lose_life(player, amount_36); }
g.end_sentence();
 Ok(()) }
fn ability_18_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_37 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_37 = { let q: &dyn Query = g; Some(asker.source) }; let duration_37 = Duration::EndOfTurn;
let kind_37_0_0 = { let q: &dyn Query = g; EffectKind::Pump((-5i64) as i16, (-5i64) as i16) };
if let Some(source) = source_37 { cantrip_api::instruction(g, |g| { for &object in &objects_37 { g.add_effect(source, object, kind_37_0_0, duration_37);  } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("ability-dungeon-room" => rooms: Room { name: "Crash Landing", effect: spell(&[], ability_0_room), leads_to: &["Goblin Camp", "Emerald Grove", "Auntie's Teahouse"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Goblin Camp", effect: spell(&[], ability_1_room), leads_to: &["Defiled Temple"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Emerald Grove", effect: spell(&[], ability_2_room), leads_to: &["Defiled Temple", "Mountain Pass"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Auntie's Teahouse", effect: spell(&[], ability_3_room), leads_to: &["Mountain Pass"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Defiled Temple", effect: spell(&[], ability_4_room), leads_to: &["Ebonlake Grotto", "Grymforge"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Mountain Pass", effect: spell(&[], ability_5_room), leads_to: &["Grymforge", "Githyanki Crèche"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Ebonlake Grotto", effect: spell(&[], ability_6_room), leads_to: &["Last Light Inn"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Grymforge", effect: spell(ABILITY_7_ROOM_TARGETS, ability_7_room), leads_to: &["Last Light Inn", "Reithwin Tollhouse"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Githyanki Crèche", effect: CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, divide: Some(Divide { total: 3, of_x: None }), ..CardDef::BLANK }, leads_to: &["Reithwin Tollhouse"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Last Light Inn", effect: spell(&[], ability_9_room), leads_to: &["Moonrise Towers", "Gauntlet of Shar"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Reithwin Tollhouse", effect: spell(&[], ability_10_room), leads_to: &["Gauntlet of Shar", "Balthazar's Lab"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Moonrise Towers", effect: spell(&[], ability_11_room), leads_to: &["Circus of the Last Days"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Gauntlet of Shar", effect: spell(&[], ability_12_room), leads_to: &["Circus of the Last Days", "Undercity Ruins"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Balthazar's Lab", effect: spell(ABILITY_13_ROOM_TARGETS, ability_13_room), leads_to: &["Undercity Ruins"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Circus of the Last Days", effect: spell(&[], ability_14_room), leads_to: &["Steel Watch Foundry", "Ansur's Sanctum"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Undercity Ruins", effect: spell(&[], ability_15_room), leads_to: &["Ansur's Sanctum", "Temple of Bhaal"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Steel Watch Foundry", effect: spell(&[], ability_16_room), leads_to: &[] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Ansur's Sanctum", effect: spell(&[], ability_17_room), leads_to: &[] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Temple of Bhaal", effect: spell(&[], ability_18_room), leads_to: &[] });
planned_item!("layout-dungeon" => dungeon: &[&[1, 2, 3], &[4], &[4, 5], &[5], &[6, 7], &[7, 8], &[9], &[9, 10], &[10], &[11, 12], &[12, 13], &[14], &[14, 15], &[15], &[16, 17], &[17, 18], &[], &[], &[]]);

pub static CARD: CardDef = CardDef { name: "Baldur's Gate Wilderness", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, lasting_cost_changes: &[LastingCostChange { text: "These spells cost less to cast.", change: lasting_cost_27, next: None }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_5::CARD,&token_7::CARD,&token_18::CARD,&token_25::CARD,&token_32::CARD,&emblem_33::CARD];
