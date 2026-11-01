// Chaos Is My Plaything
// status: planned
// requires: card type Scheme
// requires: layout Scheme
// requires: target cardinality UpTo
// requires: trigger event Player SetsSchemeInMotion
// When you set this scheme in motion, for each opponent, exile target permanent that player controls. Then each player reveals cards from the top of their library until they reveal a permanent card, puts it onto the battlefield, and puts the rest on the bottom of their library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn different_controllers_key(q: &dyn Query, id: ObjectId) -> PlayerId { q.controller(id) }
fn different_controllers(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let used: Vec<PlayerId> = choice.selected.iter().map(|&c| different_controllers_key(q, c)).collect(); let apart = used.iter().enumerate().all(|(i, k)| !used[..i].contains(k)); Verdict { complete: apart, addable: Bits::of(choice.candidates.len(), |i| !used.contains(&different_controllers_key(q, choice.candidates[i]))) } }
fn largest_different_controllers(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { cantrip_api::select::largest_distinct(choice, |c| different_controllers_key(q, c)) }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, planned_value!("target cardinality UpTo", 1u32 => TargetSpec::read_as_chosen(|q: &dyn Query, asker: &Asker| ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32))).together(cantrip_api::select::set_part(&(different_controllers as SelectFn), &(largest_different_controllers as cantrip_api::select::LargestFn), 0, "controlled by different players"))];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let owner_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_2, rest_2) = cantrip_mechanics::library::reveal_until_n(g, owner_2, count_2, &|q: &dyn Query, id: ObjectId| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))); let shown_2 = found_2.len() + rest_2.len(); let objects_2: Vec<ObjectId> = g.state().zone(Zone::library(owner_2)).iter().copied().take(shown_2).collect();
 each_3[index_1] = objects_2.clone(); all_3.extend(each_3[index_1].iter().copied());
each_4[index_1] = found_2.clone(); all_4.extend(each_4[index_1].iter().copied());
   } 
g.end_sentence();
let objects_5 = ({ let q: &dyn Query = g; (all_3.clone()).into_iter().filter(|&id| (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_5 = { let rows_5: Vec<(ObjectId, PlayerId, bool)> = objects_5.iter().map(|&o| (o, g.state().obj(o).owner, false)).collect(); let came_5 = g.put_all_onto_battlefield_as(&rows_5)?; came_5.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let library_6 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let cards_6: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (each_3[index_1].clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_6)).collect(); let placed_6 = cards_6.clone();
g.bottom_in_random_order(library_6, &cards_6);
 each_7[index_1] = placed_6.clone(); all_7.extend(each_7[index_1].iter().copied());
each_8[index_1] = { let q: &dyn Query = g; placed_6.clone() }; all_8.extend(each_8[index_1].iter().copied());
   } 
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("trigger-event-player-sets-scheme-in-motion" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SchemeSetInMotion { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-scheme" => variant: Some(Variant::Archenemy));

pub static CARD: CardDef = CardDef { name: "Chaos Is My Plaything", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Scheme", CardType::Land => CardType::Scheme)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player SetsSchemeInMotion", EventMask(0) => EventMask(EventMask::SCHEME_SET.0)), ability_0_event), ability_0, "When you set this scheme in motion, for each opponent, exile target permanent that player controls. Then each player reveals cards from the top of their library until they reveal a permanent card, puts it onto the battlefield, and puts the rest on the bottom of their library in a random order.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
