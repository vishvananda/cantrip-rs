// Wernog, Rider's Chaplain
// status: ready
// When Wernog, Rider's Chaplain enters or leaves the battlefield, each opponent may investigate. Each opponent who doesn't loses 1 life. You investigate X times, where X is one plus the number of opponents who investigated this way.
// Partner—Friends forever (You can have two commanders if both have this ability.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut guard_0_0 = vec![false; players_0.len()];
let mut each_3 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_0_0[index_0] = true;
{ let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let times = 1u32; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
let happened_2: bool = true;
 each_3[index_0] = happened_2;
  }
 } 
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_4 = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).and_then(|p: PlayerId| players_0.iter().position(|&next| next == p)).is_some_and(|at| (each_3[at] as i64) != 0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
for player in players_4 { g.lose_life(player, amount_4); }
g.end_sentence();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = { let q: &dyn Query = g; ((1i64 + (q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_0.iter().position(|&next| next == p)).is_some_and(|at| (each_3[at] as i64) != 0)).collect::<Vec<PlayerId>>()).len() as i64)).clamp(0, 255) as u32 }; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_6.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut guard_6_0 = vec![false; players_6.len()];
let mut each_9 = vec![Default::default(); players_6.len()];
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_6_0[index_6] = true;
{ let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let times = 1u32; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
let happened_8: bool = true;
 each_9[index_6] = happened_8;
  }
 } 
g.end_sentence();
let amount_10 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_10 = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).and_then(|p: PlayerId| players_6.iter().position(|&next| next == p)).is_some_and(|at| (each_9[at] as i64) != 0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
for player in players_10 { g.lose_life(player, amount_10); }
g.end_sentence();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = { let q: &dyn Query = g; ((1i64 + (q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_6.iter().position(|&next| next == p)).is_some_and(|at| (each_9[at] as i64) != 0)).collect::<Vec<PlayerId>>()).len() as i64)).clamp(0, 255) as u32 }; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wernog, Rider's Chaplain", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Wernog, Rider's Chaplain enters or leaves the battlefield, each opponent may investigate. Each opponent who doesn't loses 1 life. You investigate X times, where X is one plus the number of opponents who investigated this way.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When Wernog, Rider's Chaplain enters or leaves the battlefield, each opponent may investigate. Each opponent who doesn't loses 1 life. You investigate X times, where X is one plus the number of opponents who investigated this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::tokens::CLUE];
