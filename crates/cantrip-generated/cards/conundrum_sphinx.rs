// Conundrum Sphinx
// status: ready
// Flying
// Whenever this creature attacks, each player chooses a card name. Then each player reveals the top card of their library. If the card a player revealed has the name they chose, that player puts it into their hand. If it doesn't, that player puts it on the bottom of their library.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_chosen_1 = Vec::new();
let mut each_chosen_1_who: Vec<PlayerId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let Answer::Name(Some(chosen_1)) = g.ask(player_1, RequestKind::ChooseName { source: asker.source, looked_at: Vec::new(), suggestion: None, kind: NameKind::ANY })? else { return Ok(()) };
 each_chosen_1.push(chosen_1);
each_chosen_1_who.push(bound_0);
   } 
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_2.clone()).into_iter().filter(|&id: &ObjectId| q.state().obj(id).owner == bound_0).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| each_chosen_1.iter().zip(&each_chosen_1_who).any(|(&chosen, &who)| Some(who) == Some(bound_0) && cantrip_mechanics::library::object_has_name(q, id, chosen))).count() as i64) >= (1i64)).collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { gathered_4.extend({ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() });
    } 
let objects_6 = { let q: &dyn Query = g; gathered_4.clone() }; let moved_6 = g.return_to_hand_revealed(&objects_6);
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((objects_2.clone()).into_iter().filter(|&id: &ObjectId| q.state().obj(id).owner == bound_0).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| each_chosen_1.iter().zip(&each_chosen_1_who).any(|(&chosen, &who)| Some(who) == Some(bound_0) && cantrip_mechanics::library::object_has_name(q, id, chosen))).count() as i64) == (0i64)).collect::<Vec<PlayerId>>() };
let mut gathered_8: Vec<ObjectId> = Vec::new();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { gathered_8.extend({ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() });
    } 
let objects_10 = { let q: &dyn Query = g; gathered_8.clone() }; let moved_10 = { let order_10 = match objects_10.iter().all(|&o| g.state().obj(o).owner == asker.controller) { true => cantrip_mechanics::library::ask_library_order(g, asker.controller, asker.source, &objects_10, DestinationKind::LibraryBottom)?, false => objects_10.clone() }; g.put_into_library(&order_10, LibraryPlace::Bottom) };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Conundrum Sphinx", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, each player chooses a card name. Then each player reveals the top card of their library. If the card a player revealed has the name they chose, that player puts it into their hand. If it doesn't, that player puts it on the bottom of their library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
