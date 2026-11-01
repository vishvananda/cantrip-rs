// Krenko's Buzzcrusher
// status: ready
// Flying, trample
// When this creature enters, for each player, destroy up to one nonbasic land that player controls. For each land destroyed this way, its controller may search their library for a basic land card, put it onto the battlefield tapped, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_pool: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let mut selected_1: Vec<ObjectId> = Vec::new(); for each_player in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } { let candidates: Vec<ObjectId> = { let q: &dyn Query = g; selected_1_pool.iter().copied().filter(|&o| (if q.state().obj(o).zone.is(ZoneKind::Battlefield) { q.controller(o) } else { q.state().obj(o).owner }) == each_player).collect() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; selected_1.extend(cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Choose, candidates, cantrip_mechanics::select::Selection::of(count, &[]))?); }
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let destroyed_2 = g.destroy_all(&objects_2);
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
let mut guard_3_0 = vec![false; players_3.len()];
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_3_0[index_3] = true;
let searcher_4 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let owner_4 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_4 = cantrip_mechanics::library::searched_cards(g, owner_4); let can_4: Vec<ObjectId> = { let q: &dyn Query = g; shown_4.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_4 = cantrip_mechanics::library::search(g, searcher_4, owner_4, asker.source, SearchDestination::BattlefieldTapped, cantrip_mechanics::select::Selection::listed(rule_count_4, &shown_4, can_4))?;
 each_5[index_3] = found_4.clone(); all_5.extend(each_5[index_3].iter().copied());
  }
 } 
g.end_sentence();
let objects_6 = ({ let q: &dyn Query = g; all_5.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_6 = { let rows_6: Vec<(ObjectId, PlayerId, bool)> = objects_6.iter().map(|&o| (o, g.state().obj(o).owner, true)).collect(); let came_6 = g.put_all_onto_battlefield_as(&rows_6)?; came_6.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { if guard_3_0[index_3] {
for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
   }
 } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Krenko's Buzzcrusher", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Insect,Subtype::Thopter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, for each player, destroy up to one nonbasic land that player controls. For each land destroyed this way, its controller may search their library for a basic land card, put it onto the battlefield tapped, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
