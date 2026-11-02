// Gilded Ambusher
// status: ready
// Whenever this creature attacks, you may sacrifice another nonland permanent. When you do, each opponent exiles a nonland, nontoken permanent they control and a random nonland card from their library, then Gilded Ambusher deals damage to them equal to the total mana value of the cards exiled this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn reflexive_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_3.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let selected_4_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((!(q.chars(id).is(CardType::Land)) && !(q.state().obj(id).token)) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::ChooseAsEnters, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
if !selected_4.is_empty() { 
 each_5[index_3] = selected_4.clone(); all_5.extend(each_5[index_3].iter().copied());
 }  } 
g.end_sentence();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let mut pool_6 = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let count_6 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_6: Vec<ObjectId> = Vec::new(); while picked_6.len() < count_6 && !pool_6.is_empty() { let at = g.choose_at_random(pool_6.len()); picked_6.push(pool_6.remove(at)); }
 each_7[index_3] = picked_6.clone(); all_7.extend(each_7[index_3].iter().copied());
   } 
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in all_5.clone() { if !all.contains(&id) { all.push(id); } } for id in all_7.clone() { if !all.contains(&id) { all.push(id); } } all } }; let moved_8 = g.exile_all(&objects_8);
g.end_sentence();
let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let amount_9 = { let q: &dyn Query = g; ((moved_8.clone()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>()).clamp(0, 65535) as u16 }; let recipients_9 = { let q: &dyn Query = g; (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_9 { let batch: Vec<(ObjectId, Target, u16)> = recipients_9.iter().map(|t| (source, *t, amount_9)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(item.subject.object() == Some(id)) && !(q.chars(id).is(CardType::Land)))).collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; for player in players_0 { let eligible: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_0.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_0.clone(), &[]))? }; for object in taken { picks_0.push((player, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
if !sacrificed_0.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_10.extend(sacrificed_0.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });

}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gilded Ambusher", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard,Subtype::Glimmer]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks, you may sacrifice another nonland permanent. When you do, each opponent exiles a nonland, nontoken permanent they control and a random nonland card from their library, then Gilded Ambusher deals damage to them equal to the total mana value of the cards exiled this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_2, "When you do, each opponent exiles a nonland, nontoken permanent they control and a random nonland card from their library, then Gilded Ambusher deals damage to them equal to the total mana value of the cards exiled this way.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
