// Professor Onyx
// status: ready
// Magecraft — Whenever you cast or copy an instant or sorcery spell, each opponent loses 2 life and you gain 2 life.
// [+1]: You lose 1 life. Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.
// [−3]: Each opponent sacrifices a creature with the greatest power among creatures that player controls.
// [−8]: Each opponent may discard a card. If they don't, they lose 3 life. Repeat this process six more times.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
if let Event::Copied { copy, player, .. } = event { let id = *copy; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.lose_life(player, amount_2); }
g.end_sentence();
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_3 = { let q: &dyn Query = g; (3i64).max(0) as usize }; cantrip_mechanics::library::look_at_top_take_one_rest_to_graveyard(g, player_3, asker.source, depth_3)?;
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut gathered_5: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let objects_6 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) == ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).max().unwrap_or(0))))).collect::<Vec<ObjectId>>() }; let players_6 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_6: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_6 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_6 { let eligible: Vec<ObjectId> = objects_6.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_6.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_6.clone(), &[]))? }; for object in taken { picks_6.push((player, object)); } }
gathered_5.extend(picks_6.iter().copied());
    } 
let sacrificed_5 = g.sacrifice_each(&gathered_5);
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut runs_7: u32 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; while runs_7 > 0 { runs_7 -= 1; let mut again_7 = false;
let mut players_8: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_8_0 = vec![false; players_8.len()];
let mut each_12: Vec<Vec<ObjectId>> = vec![Vec::new(); players_8.len()]; let mut all_12: Vec<ObjectId> = Vec::new();
let mut each_13: Vec<Vec<ObjectId>> = vec![Vec::new(); players_8.len()]; let mut all_13: Vec<ObjectId> = Vec::new();
let mut each_14 = vec![Default::default(); players_8.len()];
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { let payer_9 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_9, after_9, before_9, picked_9): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_11 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_11 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_11 = Vec::new(); for player in players_11 { cards_11.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_11, true)? }); } let discarded_11 = g.discard_together(&cards_11);
let paid_10 = !discarded_11.clone().is_empty();
(paid_10, { let q: &dyn Query = g; (discarded_11).clone() }, { let q: &dyn Query = g; (discarded_11.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_12[index_8] = after_9.clone(); all_12.extend(each_12[index_8].iter().copied());
each_13[index_8] = { let q: &dyn Query = g; before_9.clone() }; all_13.extend(each_13[index_8].iter().copied());
each_14[index_8] = paid_9;
   } 
g.end_sentence();
 for (index_8, bound_0) in players_8.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_14[index_8] as i64) != 0)) } { guard_8_0[index_8] = true;
let amount_15 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_15 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_15 { g.lose_life(player, amount_15); }
   }
 } 
g.end_sentence();
again_7 = true;
g.end_sentence();
hoisted_17.extend(all_12.clone());
hoisted_18.extend({ let q: &dyn Query = g; all_13.clone() });
if !again_7 { break; } }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Professor Onyx", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Liliana]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(5), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0 | EventMask::COPIED.0), ability_0_event), ability_0, "Magecraft — Whenever you cast or copy an instant or sorcery spell, each opponent loses 2 life and you gain 2 life.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_1, "+1: You lose 1 life. Look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-3)), ability_2, "−3: Each opponent sacrifices a creature with the greatest power among creatures that player controls.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-8)), ability_3, "−8: Each opponent may discard a card. If they don't, they lose 3 life. Repeat this process six more times.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
