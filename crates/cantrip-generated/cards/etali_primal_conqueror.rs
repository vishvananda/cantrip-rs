// Etali, Primal Conqueror // Etali, Primal Sickness
// status: ready
// Trample, indestructible
// Whenever Etali deals combat damage to a player, they get that many poison counters. (A player with ten or more poison counters loses the game.)
// Trample
// When Etali enters, each player exiles cards from the top of their library until they exile a nonland card. You may cast any number of spells from among the nonland cards exiled this way without paying their mana costs.
// {9}{G/P}: Transform Etali. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let owner_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_1, rest_1) = cantrip_mechanics::library::reveal_until_n(g, owner_1, count_1, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_1 = found_1.len() + rest_1.len(); let objects_1: Vec<ObjectId> = g.state().zone(Zone::library(owner_1)).iter().copied().take(shown_1).collect();
 each_2[index_0] = objects_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_3[index_0] = found_1.clone(); all_3.extend(each_3[index_0].iter().copied());
   } 
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; all_2.clone() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
let caster_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_5: Vec<ObjectId> = { let q: &dyn Query = g; (moved_4.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_5: u32 = 4294967295; let mut played_5: Vec<ObjectId> = Vec::new();
while left_5 > 0 {
let mut open_5: Vec<ObjectId> = Vec::new(); for &c in &pool_5 { let ways = g.cast_ways_during_resolution_by(c, caster_5); if ways.iter().any(|w| !g.card(w.face).types.has(CardType::Land)) { open_5.push(c); } }
let picked_5 = if open_5.is_empty() { None } else if open_5.len() > 1 || (true && (!false || pool_5.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_5, asker.source, Purpose::CastFree, pool_5.clone(), open_5.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_5, open_5[0], Purpose::CastFree)? { Some(open_5[0]) } else { None };
let Some(card_5) = picked_5 else { break; };
let ways_5: Vec<CastWay> = g.cast_ways_during_resolution_by(card_5, caster_5).into_iter().filter(|w| !g.card(w.face).types.has(CardType::Land)).collect();
let done_5 = if ways_5.is_empty() { g.play_land_during_resolution(card_5)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_5, card_5, ways_5)?.map(|w| w.way); g.cast_during_resolution_as(card_5, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_5), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_5 { left_5 -= 1; played_5.push(g.state().obj(card_5).became.unwrap_or(card_5)); } else if false { break; } if done_5 || !(true && true) { pool_5.retain(|&c| c != card_5); }
}
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Etali, Primal Conqueror", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elder,Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Etali enters, each player exiles cards from the top of their library until they exile a nonland card. You may cast any number of spells from among the nonland cards exiled this way without paying their mana costs.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 1] })), ability_2, "{9}{G/P}: Transform Etali. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (i64::from(item.data[0])) as u32 }; for player in { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() } { g.give_poison(player, amount); }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Etali, Primal Sickness", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Elder,Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(24)), power: Some(11), toughness: Some(11), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample,Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Etali deals combat damage to a player, they get that many poison counters.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
