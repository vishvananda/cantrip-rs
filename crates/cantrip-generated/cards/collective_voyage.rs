// Collective Voyage
// status: ready
// Join forces — Starting with you, each player may pay any amount of mana. Each player searches their library for up to X basic land cards, where X is the total amount of mana paid this way, puts them onto the battlefield tapped, then shuffles.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
let mut guard_0_0 = vec![false; players_0.len()];
let mut each_2 = vec![Default::default(); players_0.len()];
let mut done_total_2: i64 = 0;
let mut each_3 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_0_0[index_0] = true;
let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { i64::from(g.may_pay_x(player, asker.source, asker.source, cost("{X}"), &PAY_QUESTION)?.unwrap_or(0)) }; }
 each_2[index_0] = done_1;
done_total_2 += done_1;
each_3[index_0] = done_1;
  }
 } 
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let searcher_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let owner_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let rule_count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo(((done_total_2 as i64)) as u32) }; let shown_5 = cantrip_mechanics::library::searched_cards(g, owner_5); let can_5: Vec<ObjectId> = { let q: &dyn Query = g; shown_5.iter().copied().filter(|&id| (q.chars(id).supertypes.basic && q.chars(id).is(CardType::Land))).collect() }; let found_5 = cantrip_mechanics::library::search(g, searcher_5, owner_5, asker.source, SearchDestination::BattlefieldTapped, cantrip_mechanics::select::Selection::listed(rule_count_5, &shown_5, can_5))?;
 each_6[index_4] = found_5.clone(); all_6.extend(each_6[index_4].iter().copied());
   } 
g.end_sentence();
let objects_7 = ({ let q: &dyn Query = g; all_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, g.state().obj(o).owner, true)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
    } 
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Collective Voyage", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
