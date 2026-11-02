// Sculpted Sunburst
// status: ready
// Choose a creature you control, then each opponent chooses a creature they control with equal or lesser power. If you chose a creature this way, exile each creature not chosen by any player this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::ChooseAsEnters, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_1.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let selected_2_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)) && q.chars(id).power.is_some_and(|v| (i64::from(v)) <= ((selected_0.clone()).get(0 as usize).copied().map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))))))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::ChooseAsEnters, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
 each_3[index_1] = selected_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
g.end_sentence();
if { let q: &dyn Query = g; (selected_0.clone()).get(0 as usize).copied().is_some_and(|candidate: ObjectId| q.chars(candidate).is(CardType::Creature)) } {
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !(({ let mut all: Vec<ObjectId> = Vec::new(); for id in selected_0.clone() { if !all.contains(&id) { all.push(id); } } for id in all_3.clone() { if !all.contains(&id) { all.push(id); } } all }).contains(&id)))).collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sculpted Sunburst", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
