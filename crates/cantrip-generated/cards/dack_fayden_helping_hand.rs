// Dack Fayden, Helping Hand
// status: planned
// requires: object reference Earlier
// When Dack Fayden enters, reveal cards from the top of your library until you reveal X creature cards, where X is the number of opponents you have. Put those creature cards onto the battlefield, then shuffle. They're goaded for the rest of the game. For each of those permanents, choose a different opponent. Each opponent gains control of the permanent for which they were chosen.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).len() as i64).max(0) as usize }; let (found_0, rest_0) = cantrip_mechanics::library::reveal_until_n(g, owner_0, count_0, &|q: &dyn Query, id: ObjectId| q.chars(id).is(CardType::Creature)); let shown_0 = found_0.len() + rest_0.len(); let objects_0: Vec<ObjectId> = g.state().zone(Zone::library(owner_0)).iter().copied().take(shown_0).collect();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; (objects_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, false)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (moved_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let duration_3 = Duration::Indefinite; for object in objects_3 { cantrip_mechanics::goad::goad_for(g, item, object, duration_3); }
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_4.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_4, _) in players_4.iter().copied().enumerate() { let selected_5_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_5_candidates = { let q: &dyn Query = g; moved_1.clone() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::ChooseAsEnters, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
 each_6[index_4] = selected_5.clone(); all_6.extend(each_6[index_4].iter().copied());
   } 
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_7.sort_by_key(|p| order.iter().position(|o| o == p)); }
let snap_8: Vec<Vec<ObjectId>> = players_7.iter().copied().map(|bound_0| { let q: &dyn Query = g; planned_value!("object reference Earlier", None::<ObjectId> => q.earlier_object(item, EarlierObjects::Selected, Some(bound_0))).into_iter().collect::<Vec<ObjectId>>() }).collect();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_7.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
g.one_instruction(&mut |g| { for (index_7, bound_0) in players_7.iter().copied().enumerate() { let objects_9 = { let q: &dyn Query = g; (snap_8[index_7].clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let duration_9 = Duration::Indefinite;
let kind_9_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(bound_0).expect("a reference that always names a player")) };
if let Some(source) = source_9 { cantrip_api::instruction(g, |g| { for &object in &objects_9 { g.add_effect(source, object, kind_9_0_0, duration_9);  } }); }
 each_10[index_7] = objects_9.clone(); all_10.extend(each_10[index_7].iter().copied());
   } Ok(()) })?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Dack Fayden, Helping Hand", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Dack Fayden enters, reveal cards from the top of your library until you reveal X creature cards, where X is the number of opponents you have. Put those creature cards onto the battlefield, then shuffle. They're goaded for the rest of the game. For each of those permanents, choose a different opponent. Each opponent gains control of the permanent for which they were chosen.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
