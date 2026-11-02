// From the Ashes
// status: ready
// Destroy all nonbasic lands. For each land destroyed this way, its controller may search their library for a basic land card and put it onto the battlefield. Then each player who searched their library this way shuffles.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64)).collect::<Vec<PlayerId>>() };
let mut guard_1_0 = vec![false; players_1.len()];
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_1_0[index_1] = true;
let searcher_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let owner_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_0) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).len() as i64) as u32) }; let shown_2 = cantrip_mechanics::library::searched_cards(g, owner_2); let can_2: Vec<ObjectId> = { let q: &dyn Query = g; shown_2.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_2 = cantrip_mechanics::library::search(g, searcher_2, owner_2, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(rule_count_2, &shown_2, can_2))?;
 each_3[index_1] = found_2.clone(); all_3.extend(each_3[index_1].iter().copied());
  }
 } 
g.end_sentence();
let objects_4 = ({ let q: &dyn Query = g; all_3.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if guard_1_0[index_1] {
for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
   }
 } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "From the Ashes", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
