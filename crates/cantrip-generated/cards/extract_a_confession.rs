// Extract a Confession
// status: ready
// As an additional cost to cast this spell, you may collect evidence 6. (Exile cards with total mana value 6 or greater from your graveyard.)
// Each opponent sacrifices a creature of their choice. If evidence was collected, instead each opponent sacrifices a creature with the greatest power among creatures they control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (i64::from(q.state().obj(asker.source).paid)) >= (1i64) } {
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_1: Vec<(PlayerId, ObjectId)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).power.is_some_and(|v| (i64::from(v)) == ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter_map(|o: ObjectId| q.chars(o).power).map(i64::from).max().unwrap_or(0))))).collect::<Vec<ObjectId>>() }; let players_2 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_2: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_2 { let eligible: Vec<ObjectId> = objects_2.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_2.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_2.clone(), &[]))? }; for object in taken { picks_2.push((player, object)); } }
gathered_1.extend(picks_2.iter().copied());
    } 
let sacrificed_1 = g.sacrifice_each(&gathered_1);
g.end_sentence();
hoisted_3.extend(sacrificed_1.clone());
hoisted_4.extend({ let q: &dyn Query = g; gathered_1.iter().map(|pick| pick.1).collect::<Vec<ObjectId>>().iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_5 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_5 { let eligible: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_5.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_5.clone(), &[]))? }; for object in taken { picks_5.push((player, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
hoisted_6.extend(sacrificed_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Extract a Confession", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Cast without collecting", "Collect evidence 6"], optional_cost: Some(OptionalCost::new(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { collect_evidence: 6, ..Cost::NONE }))), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
