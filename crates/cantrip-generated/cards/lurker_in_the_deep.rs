// Lurker in the Deep
// status: planned
// requires: earlier objects Sought
// requires: operation Conjure
// requires: trigger event Player Seeks
// Impending 3—{2}{U}{U}
// Whenever this permanent enters or attacks, seek a nonland card.
// Whenever you seek one or more cards during your turn, conjure a duplicate of each of those cards into your hand, then manifest those duplicates.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_0: Vec<ObjectId> = Vec::new(); while picked_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); picked_0.push(pool_0.remove(at)); }
let objects_1 = { let q: &dyn Query = g; picked_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let duplicated_2 = { let q: &dyn Query = g; planned_value!("earlier objects Sought", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Sought)) };
let count_2: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_2: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_2.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_2), count_2, ConjureTo::Hand)?)); }
g.end_sentence();
let controller_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let cards_3 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (conjured_2.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let entered_3: Vec<ObjectId> = g.put_all_onto_battlefield_face_down(&cards_3, controller_3, FaceDown::Manifested, false)?.into_iter().flatten().collect();
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-seeks" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Sought { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Lurker in the Deep", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fish,Subtype::Illusion]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::impending::REMOVE_A_TIME_COUNTER,triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this permanent enters or attacks, seek a nonland card.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player Seeks", EventMask(0) => EventMask(EventMask::SOUGHT.0)), ability_2_event), ability_2, "Whenever you seek one or more cards during your turn, conjure a duplicate of each of those cards into your hand, then manifest those duplicates.").targets(&[])], statics: &[cantrip_mechanics::impending::NOT_A_CREATURE], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(cantrip_mechanics::impending::enters::<3>), modal: None, enchant: None, ways: &[cantrip_mechanics::impending::way(ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, "Impending 3—{2}{U}{U}")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
