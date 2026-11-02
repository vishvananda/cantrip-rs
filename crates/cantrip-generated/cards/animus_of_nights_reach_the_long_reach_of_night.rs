// Animus of Night's Reach // The Long Reach of Night
// status: ready
// Menace (This creature can't be blocked except by two or more creatures.)
// Whenever this creature attacks, it gets +X/+0 until end of turn, where X is the number of creature cards in defending player's graveyard.
// (As this Saga enters and after your draw step, add a lore counter.)
// I, II — Each opponent sacrifices a creature of their choice unless they discard a card.
// III — Exile this Saga, then return it to the battlefield transformed under your control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let payer_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_3, true)? }); } let discarded_3 = g.discard_together(&cards_3);
let paid_2 = !discarded_3.clone().is_empty();
(paid_2, { let q: &dyn Query = g; (discarded_3).clone() }, { let q: &dyn Query = g; (discarded_3.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_4[index_0] = after_1.clone(); all_4.extend(each_4[index_0].iter().copied());
each_5[index_0] = { let q: &dyn Query = g; before_1.clone() }; all_5.extend(each_5[index_0].iter().copied());
each_6[index_0] = paid_1;
   } 
g.end_sentence();
let mut gathered_7: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_6[index_0] as i64) != 0)) } { guard_0_0[index_0] = true;
let objects_8 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_8 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_8: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_8 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_8 { let eligible: Vec<ObjectId> = objects_8.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_8.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_8.clone(), &[]))? }; for object in taken { picks_8.push((player, object)); } }
gathered_7.extend(picks_8.iter().copied());
   }
 } 
let sacrificed_7 = g.sacrifice_each(&gathered_7);
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_9_0 = vec![false; players_9.len()];
let mut each_13: Vec<Vec<ObjectId>> = vec![Vec::new(); players_9.len()]; let mut all_13: Vec<ObjectId> = Vec::new();
let mut each_14: Vec<Vec<ObjectId>> = vec![Vec::new(); players_9.len()]; let mut all_14: Vec<ObjectId> = Vec::new();
let mut each_15 = vec![Default::default(); players_9.len()];
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let payer_10 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_10, after_10, before_10, picked_10): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_12 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_12: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_12 = Vec::new(); for player in players_12 { cards_12.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_12, true)? }); } let discarded_12 = g.discard_together(&cards_12);
let paid_11 = !discarded_12.clone().is_empty();
(paid_11, { let q: &dyn Query = g; (discarded_12).clone() }, { let q: &dyn Query = g; (discarded_12.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_13[index_9] = after_10.clone(); all_13.extend(each_13[index_9].iter().copied());
each_14[index_9] = { let q: &dyn Query = g; before_10.clone() }; all_14.extend(each_14[index_9].iter().copied());
each_15[index_9] = paid_10;
   } 
g.end_sentence();
let mut gathered_16: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_15[index_9] as i64) != 0)) } { guard_9_0[index_9] = true;
let objects_17 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_17 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_17: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_17 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_17 { let eligible: Vec<ObjectId> = objects_17.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_17.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_17.clone(), &[]))? }; for object in taken { picks_17.push((player, object)); } }
gathered_16.extend(picks_17.iter().copied());
   }
 } 
let sacrificed_16 = g.sacrifice_each(&gathered_16);
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_18 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_18 = g.exile_all(&objects_18);
g.end_sentence();
let player_19 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_19 = ({ let q: &dyn Query = g; ((moved_18.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_19 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_19 { let controller = player_19; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Long Reach of Night", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II — Each opponent sacrifices a creature of their choice unless they discard a card.").targets(&[]),chapter(2, ability_0_1, "I, II — Each opponent sacrifices a creature of their choice unless they discard a card.").targets(&[]),chapter(3, ability_0_2, "III — Exile this Saga, then return it to the battlefield transformed under your control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && item.subject.object().and_then(|o: ObjectId| q.defending_player(o)) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16, (0i64) as i16) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Animus of Night's Reach", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, it gets +X/+0 until end of turn, where X is the number of creature cards in defending player's graveyard.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
