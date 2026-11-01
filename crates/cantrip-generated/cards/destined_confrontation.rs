// Destined Confrontation
// status: ready
// Each player chooses any number of creatures they control with total power 4 or less, then sacrifices all other creatures they control.
#[allow(unused_imports)] use cantrip_api::*;
fn total_power_of(q: &dyn Query, id: ObjectId) -> u64 { i64::from(q.chars(id).power.unwrap_or(0)).max(0) as u64 }
fn total_power(q: &dyn Query, choice: &Choice, most: u32) -> Verdict { let total: u64 = choice.selected.iter().map(|&c| total_power_of(q, c)).sum(); Verdict { complete: total <= u64::from(most), addable: Bits::of(choice.candidates.len(), |i| total + total_power_of(q, choice.candidates[i]) <= u64::from(most)) } }
fn largest_total_power(q: &dyn Query, choice: &Choice, most: u32) -> Vec<ObjectId> { cantrip_api::select::largest_within(choice, |c| total_power_of(q, c), u64::from(most)) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let pool_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let picked_1 = cantrip_mechanics::choose::select(g, player_1, asker.source, Purpose::Keep, pool_1, cantrip_mechanics::select::Selection::of(count_1, &[]).with(cantrip_api::select::set_part(&(total_power as SelectFn), &(largest_total_power as cantrip_api::select::LargestFn), 4, "with total power {n} or less")))?;
 each_2[index_0] = picked_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; { let chosen: Vec<ObjectId> = all_2.clone(); (players_0.iter().copied().flat_map(|bound_0: PlayerId| q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() } }; let players_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Destined Confrontation", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
