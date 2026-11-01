// Kroxa, Titan of Death's Hunger
// status: planned
// requires: numeric expression Earlier
// When Kroxa enters, sacrifice it unless it escaped.
// Whenever Kroxa enters or attacks, each opponent discards a card, then each opponent who didn't discard a nonland card this way loses 3 life.
// Escape—{B}{B}{R}{R}, Exile five other cards from your graveyard. (You may cast this card from your graveyard for its escape cost.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let mut hoisted_1: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; !(item.subject.object().is_some_and(|candidate: ObjectId| cantrip_mechanics::escape::escaped(q, candidate))) } {
let objects_0 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_0 { let controller = g.controller(object); if players_0.contains(&controller) && g.can_sacrifice(controller, object) { picks_0.push((controller, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
g.end_sentence();
hoisted_1.extend(sacrificed_0.clone());
hoisted_2.extend({ let q: &dyn Query = g; objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
let mut each_8 = vec![Default::default(); players_3.len()];
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_5 = Vec::new(); for player in players_5 { cards_5.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_5)?); } let discarded_5 = g.discard_together(&cards_5);
let paid_4 = !discarded_5.clone().is_empty();
 each_6[index_3] = discarded_5.clone(); all_6.extend(each_6[index_3].iter().copied());
each_7[index_3] = { let q: &dyn Query = g; discarded_5.clone() }; all_7.extend(each_7[index_3].iter().copied());
each_8[index_3] = paid_4;
   } 
g.end_sentence();
let amount_9 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_9 = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| (planned_value!("numeric expression Earlier", 0i64 => q.earlier_objects(item, EarlierObjects::Discarded, Some(bound_0)).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).count() as i64)) == (0i64)).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }; all.retain(|p: &PlayerId| other.contains(p)); all } };
for player in players_9 { g.lose_life(player, amount_9); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Kroxa, Titan of Death's Hunger", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elder,Subtype::Giant]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Kroxa enters, sacrifice it unless it escaped.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Kroxa enters or attacks, each opponent discards a card, then each opponent who didn't discard a nonland card this way loses 3 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[cantrip_mechanics::escape::way(ManaCost { generic: 0, x: 0, symbols: [0, 0, 2, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, 5, "Escape—{B}{B}{R}{R}, Exile five other cards from your graveyard.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
