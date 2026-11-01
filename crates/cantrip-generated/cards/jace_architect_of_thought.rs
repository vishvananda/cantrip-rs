// Jace, Architect of Thought
// status: ready
// [+1]: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.
// [−2]: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.
// [−8]: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Pump((-1i64) as i16, (0i64) as i16) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }), source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((3i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_3: Vec<PlayerId> = objects_3.iter().map(|o| g.state().obj(*o).owner).collect(); owners_3.dedup(); for owner in owners_3 { let cards: Vec<ObjectId> = objects_3.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if !among.is_empty() { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let cards_5 = { let q: &dyn Query = g; (objects_3.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let separator_5 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let chooser_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (first_5, second_5) = cantrip_mechanics::choose::separate(g, Some(separator_5), asker.source, cards_5)?; let (pile_5, other_5) = cantrip_mechanics::choose::pile(g, chooser_5, asker.source, first_5, second_5)?;

let objects_6 = { let q: &dyn Query = g; pile_5.clone() }; let moved_6 = g.return_to_hand_revealed(&objects_6);
let library_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_7: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_3.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_7)).collect(); let placed_7 = cards_7.clone();
let chooser_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_7 = cantrip_mechanics::library::ask_library_order(g, chooser_7, asker.source, &cards_7, DestinationKind::LibraryBottom)?; g.arrange_library(library_7, &[], &order_7);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_8: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_8.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { let searcher_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_9 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_9 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_9 = cantrip_mechanics::library::searched_cards(g, owner_9); let can_9: Vec<ObjectId> = { let q: &dyn Query = g; shown_9.iter().copied().filter(|&id| !(q.chars(id).is(CardType::Land))).collect() }; let found_9 = cantrip_mechanics::library::search(g, searcher_9, owner_9, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(rule_count_9, &shown_9, can_9))?;
 each_10[index_8] = found_9.clone(); all_10.extend(each_10[index_8].iter().copied());
   } 
g.end_sentence();
let objects_11 = { let q: &dyn Query = g; all_10.clone() }; let moved_11 = g.exile_all(&objects_11);
g.end_sentence();
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
let caster_13 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_13: Vec<ObjectId> = { let q: &dyn Query = g; (moved_11.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_13: u32 = 4294967295; let mut played_13: Vec<ObjectId> = Vec::new();
while left_13 > 0 {
let mut open_13: Vec<ObjectId> = Vec::new(); for &c in &pool_13 { let ways = g.cast_ways_during_resolution_by(c, caster_13); if ways.iter().any(|w| true) { open_13.push(c); } }
let picked_13 = if open_13.is_empty() { None } else if open_13.len() > 1 || (false && (!false || pool_13.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_13, asker.source, Purpose::CastFree, pool_13.clone(), open_13.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_13, open_13[0], Purpose::CastFree)? { Some(open_13[0]) } else { None };
let Some(card_13) = picked_13 else { break; };
let ways_13: Vec<CastWay> = g.cast_ways_during_resolution_by(card_13, caster_13).into_iter().filter(|w| true).collect();
let done_13 = if ways_13.is_empty() { g.play_land_during_resolution(card_13)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_13, card_13, ways_13)?.map(|w| w.way); g.cast_during_resolution_as(card_13, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_13), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_13 { left_13 -= 1; played_13.push(g.state().obj(card_13).became.unwrap_or(card_13)); } else if 4294967295i64 == 1 { break; } if done_13 || !(true && false) { pool_13.retain(|&c| c != card_13); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Jace, Architect of Thought", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Jace]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(4), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_0, "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_1, "−2: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-8)), ability_2, "−8: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), watch_1_event), watch_1, "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
