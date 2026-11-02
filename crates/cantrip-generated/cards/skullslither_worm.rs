// Skullslither Worm
// status: ready
// When this creature enters, each opponent discards a card. For each opponent who can't, put two +1/+1 counters on this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_2)?); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
 each_3[index_0] = discarded_2.clone(); all_3.extend(each_3[index_0].iter().copied());
each_4[index_0] = { let q: &dyn Query = g; discarded_2.clone() }; all_4.extend(each_4[index_0].iter().copied());
each_5[index_0] = paid_1;
   } 
g.end_sentence();
let mut counters_6: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; ((2i64 * ({ let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).and_then(|p: PlayerId| players_0.iter().position(|&next| next == p)).is_some_and(|at| (each_5[at] as i64) != 0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }; all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64)).clamp(0, 32767) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_6.push((object, CounterKind::PlusOne, amount)); } }
let had_6: Vec<_> = counters_6.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_6); let applied_6: Vec<ObjectId> = counters_6.iter().zip(had_6).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Skullslither Worm", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Worm]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, each opponent discards a card. For each opponent who can't, put two +1/+1 counters on this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
