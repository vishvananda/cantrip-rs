// Jhoira of the Ghitu Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// {3}, Discard a card: Copy three instant cards chosen at random. You may cast one of the copies without paying its mana cost.
// {3}, Discard a card: Copy three sorcery cards chosen at random. You may cast one of the copies without paying its mana cost. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| q.chars(id).is(CardType::Instant)).collect::<Vec<ObjectId>>() };
let count_0 = { let q: &dyn Query = g; 3i64 };
let mut random_0: Vec<ObjectId> = Vec::new(); for _ in 0..count_0 { let Some(one) = cantrip_mechanics::library::random_of(g, &pool_0) else { break }; pool_0.retain(|&o| o != one); random_0.push(one); }
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_1 = { let q: &dyn Query = g; random_0.clone() }; let times_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_1: Vec<ObjectId> = Vec::new(); for _ in 0..times_1 { for &card in &originals_1 { copies_1.push(g.create_copy_in_exile(card, player_1)); } }
g.end_sentence();
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (copies_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| true) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (true && (!true || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::CastFree, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::CastFree)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| true).collect();
let done_2 = if ways_2.is_empty() { g.play_land_during_resolution(card_2)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_2), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else if 1i64 == 1 { break; } if done_2 || !(true && true) { pool_2.retain(|&c| c != card_2); }
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_3: Vec<ObjectId> = { let q: &dyn Query = g; (q.outside_the_game(asker.controller)).into_iter().filter(|&id| q.chars(id).is(CardType::Sorcery)).collect::<Vec<ObjectId>>() };
let count_3 = { let q: &dyn Query = g; 3i64 };
let mut random_3: Vec<ObjectId> = Vec::new(); for _ in 0..count_3 { let Some(one) = cantrip_mechanics::library::random_of(g, &pool_3) else { break }; pool_3.retain(|&o| o != one); random_3.push(one); }
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_4 = { let q: &dyn Query = g; random_3.clone() }; let times_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_4: Vec<ObjectId> = Vec::new(); for _ in 0..times_4 { for &card in &originals_4 { copies_4.push(g.create_copy_in_exile(card, player_4)); } }
g.end_sentence();
let caster_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_5: Vec<ObjectId> = { let q: &dyn Query = g; (copies_4.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_5: u32 = 1; let mut played_5: Vec<ObjectId> = Vec::new();
while left_5 > 0 {
let mut open_5: Vec<ObjectId> = Vec::new(); for &c in &pool_5 { let ways = g.cast_ways_during_resolution_by(c, caster_5); if ways.iter().any(|w| true) { open_5.push(c); } }
let picked_5 = if open_5.is_empty() { None } else if open_5.len() > 1 || (true && (!true || pool_5.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_5, asker.source, Purpose::CastFree, pool_5.clone(), open_5.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_5, open_5[0], Purpose::CastFree)? { Some(open_5[0]) } else { None };
let Some(card_5) = picked_5 else { break; };
let ways_5: Vec<CastWay> = g.cast_ways_during_resolution_by(card_5, caster_5).into_iter().filter(|w| true).collect();
let done_5 = if ways_5.is_empty() { g.play_land_during_resolution(card_5)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_5, card_5, ways_5)?.map(|w| w.way); g.cast_during_resolution_as(card_5, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_5), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_5 { left_5 -= 1; played_5.push(g.state().obj(card_5).became.unwrap_or(card_5)); } else if 1i64 == 1 { break; } if done_5 || !(true && true) { pool_5.retain(|&c| c != card_5); }
}
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: 0 }));

pub static CARD: CardDef = CardDef { name: "Jhoira of the Ghitu Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard_cards(1)), ability_0, "{3}, Discard a card: Copy three instant cards chosen at random. You may cast one of the copies without paying its mana cost.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard_cards(1)), ability_1, "{3}, Discard a card: Copy three sorcery cards chosen at random. You may cast one of the copies without paying its mana cost. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
