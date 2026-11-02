// The Deck of Many Things
// status: ready
// {2}, {T}: Roll a d20 and subtract the number of cards in your hand. If the result is 0 or less, discard your hand.
// 1—9 | Return a card at random from your graveyard to your hand.
// 10—19 | Draw two cards.
// 20 | Put a creature card from any graveyard onto the battlefield under your control. When that creature dies, its owner loses the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_14_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn watch_14(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.state().obj(o).owner).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_17: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0).saturating_sub((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64)) <= (0i64) } {
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
hoisted_2.extend(discarded_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; discarded_1.clone() });
}
if { let q: &dyn Query = g; (((value_0).saturating_sub((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64)) >= (1i64) && ((value_0).saturating_sub((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64)) <= (9i64)) } {
let mut pool_4 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let count_4 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_4: Vec<ObjectId> = Vec::new(); while picked_4.len() < count_4 && !pool_4.is_empty() { let at = g.choose_at_random(pool_4.len()); picked_4.push(pool_4.remove(at)); }
let objects_5 = { let q: &dyn Query = g; picked_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
g.end_sentence();
hoisted_6.extend(moved_5.clone());
hoisted_7.extend(picked_4.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
if { let q: &dyn Query = g; (((value_0).saturating_sub((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64)) >= (10i64) && ((value_0).saturating_sub((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64)) <= (19i64)) } {
let amount_9 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.draw(player, amount_9); }
g.end_sentence();
}
if { let q: &dyn Query = g; ((value_0).saturating_sub((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64)) == (20i64) } {
let selected_10_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_10_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_10_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_10 = { cantrip_mechanics::choose::select(g, selected_10_player, asker.source, Purpose::PutOntoBattlefield, selected_10_candidates, cantrip_mechanics::select::Selection::of(selected_10_count, &[]))? };
let player_11 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_11 = ({ let q: &dyn Query = g; selected_10.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_11 = { let rows_11: Vec<(ObjectId, PlayerId, bool)> = objects_11.iter().map(|&o| (o, player_11, false)).collect(); let came_11 = g.put_all_onto_battlefield_as(&rows_11)?; came_11.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
if let Some(source) = { let q: &dyn Query = g; (moved_11.clone()).get(0 as usize).copied() } { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_16.extend(moved_11.clone());
hoisted_17.extend(selected_10.clone());
hoisted_18.extend({ let q: &dyn Query = g; objects_11.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Deck of Many Things", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{2}, {T}: Roll a d20 and subtract the number of cards in your hand. If the result is 0 or less, discard your hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), watch_14_event), watch_14, "{2}, {T}: Roll a d20 and subtract the number of cards in your hand. If the result is 0 or less, discard your hand.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
