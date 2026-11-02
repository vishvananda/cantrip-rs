// Jace, Architect of Thought
// status: ready
// [+1]: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.
// [−2]: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.
// [−8]: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::Pump((-1i64) as i16, (0i64) as i16) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }), source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((3i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_4: Vec<PlayerId> = objects_4.iter().map(|o| g.state().obj(*o).owner).collect(); owners_4.dedup(); for owner in owners_4 { let cards: Vec<ObjectId> = objects_4.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if !among.is_empty() { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let cards_6 = { let q: &dyn Query = g; (objects_4.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let separator_6 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let chooser_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (first_6, second_6) = cantrip_mechanics::choose::separate(g, Some(separator_6), asker.source, cards_6)?; let (pile_6, other_6) = cantrip_mechanics::choose::pile(g, chooser_6, asker.source, first_6, second_6)?;

let objects_7 = { let q: &dyn Query = g; pile_6.clone() }; let moved_7 = g.return_to_hand_revealed(&objects_7);
let library_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_8: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_4.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_8)).collect(); let placed_8 = cards_8.clone();
let chooser_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let order_8 = cantrip_mechanics::library::ask_library_order(g, chooser_8, asker.source, &cards_8, DestinationKind::LibraryBottom)?; g.arrange_library(library_8, &[], &order_8);
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_9.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let searcher_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_10 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_10 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_10 = cantrip_mechanics::library::searched_cards(g, owner_10); let can_10: Vec<ObjectId> = { let q: &dyn Query = g; shown_10.iter().copied().filter(|&id| !(q.chars(id).is(CardType::Land))).collect() }; let found_10 = cantrip_mechanics::library::search(g, searcher_10, owner_10, asker.source, SearchDestination::Exile, cantrip_mechanics::select::Selection::listed(rule_count_10, &shown_10, can_10))?;
 each_11[index_9] = found_10.clone(); all_11.extend(each_11[index_9].iter().copied());
   } 
g.end_sentence();
let objects_12 = { let q: &dyn Query = g; all_11.clone() }; let moved_12 = g.exile_all(&objects_12);
g.end_sentence();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
let caster_14 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_14: Vec<ObjectId> = { let q: &dyn Query = g; (moved_12.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_14: u32 = 4294967295; let mut played_14: Vec<ObjectId> = Vec::new();
while left_14 > 0 {
let mut open_14: Vec<ObjectId> = Vec::new(); for &c in &pool_14 { let ways = g.cast_ways_during_resolution_by(c, caster_14); if ways.iter().any(|w| true) { open_14.push(c); } }
let picked_14 = if open_14.is_empty() { None } else if cantrip_mechanics::effects::ask_may(g, caster_14, open_14[0], Purpose::CastFree)? { Some(open_14[0]) } else { None };
let Some(card_14) = picked_14 else { break; };
let ways_14: Vec<CastWay> = g.cast_ways_during_resolution_by(card_14, caster_14).into_iter().filter(|w| true).collect();
let done_14 = { let way = cantrip_mechanics::choose::cast_way(g, caster_14, card_14, ways_14)?.map(|w| w.way); g.cast_during_resolution_as(card_14, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_14), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_14 { left_14 -= 1; played_14.push(g.state().obj(card_14).became.unwrap_or(card_14)); } pool_14.retain(|&c| c != card_14);
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Jace, Architect of Thought", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Jace]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(4), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_0, "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_1, "−2: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-8)), ability_2, "−8: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), watch_2_event), watch_2, "Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
