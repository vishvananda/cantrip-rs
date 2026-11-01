// Regna's Sanction
// status: planned
// requires: a value chosen by several players
// requires: condition VotedFor
// requires: per-player PutCounters
// requires: per-player Tap
// For each player, choose friend or foe. Each friend puts a +1/+1 counter on each creature they control. Each foe chooses one untapped creature they control, then taps the rest.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("a-value-chosen-by-several-players" => cantrip_mechanics::choose::each_chooses(g, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Word)?);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() };
planned!("per-player-put-counters" => g.begin_batch(););
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
 each_3[index_1] = applied_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
planned!("per-player-put-counters" => g.end_batch(););
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() };
planned!("per-player-tap" => g.begin_batch(););
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let selected_6_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::Choose, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
let objects_7 = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = selected_6.clone(); (q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_7: Vec<ObjectId> = objects_7.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_7);
 each_8[index_4] = objects_7.clone(); all_8.extend(each_8[index_4].iter().copied());
each_9[index_4] = selected_6.clone(); all_9.extend(each_9[index_4].iter().copied());
each_10[index_4] = turned_7.clone(); all_10.extend(each_10[index_4].iter().copied());
each_11[index_4] = { let q: &dyn Query = g; turned_7.clone() }; all_11.extend(each_11[index_4].iter().copied());
   } 
planned!("per-player-tap" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Regna's Sanction", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
