// Polygraph Orb
// status: ready
// When this artifact enters, look at the top four cards of your library. Put two of them into your hand and the rest into your graveyard. You lose 2 life.
// {2}, {T}, Collect evidence 3: Each opponent loses 3 life unless they discard a card or sacrifice a creature. (To collect evidence 3, exile cards with total mana value 3 or greater from your graveyard.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_2 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_2, &objects_2);
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; (objects_2.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::PutIntoHand, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let objects_4 = { let q: &dyn Query = g; selected_3.clone() }; let moved_4 = g.return_to_hand(&objects_4);
let objects_5 = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (objects_2.clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let moved_5 = { let moves: Vec<(ObjectId, Zone)> = objects_5.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.lose_life(player, amount_6); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn paid_9(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_7.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut guard_7_0 = vec![false; players_7.len()];
let mut each_10 = vec![Default::default(); players_7.len()];
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { let payer_8 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let paid_8 = g.may_pay(payer_8, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(discard_cards(1)).and(sacrifice(1, paid_9)), &PAY_QUESTION)?;
 each_10[index_7] = paid_8;
   } 
g.end_sentence();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_10[index_7] as i64) != 0)) } { guard_7_0[index_7] = true;
let amount_11 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_11 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_11 { g.lose_life(player, amount_11); }
   }
 } 
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Polygraph Orb", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this artifact enters, look at the top four cards of your library. Put two of them into your hand and the rest into your graveyard. You lose 2 life.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(Cost { collect_evidence: 3, ..Cost::NONE }), ability_1, "{2}, {T}, Collect evidence 3: Each opponent loses 3 life unless they discard a card or sacrifice a creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
