// Quest for Ula's Temple
// status: ready
// At the beginning of your upkeep, you may look at the top card of your library. If it's a creature card, you may reveal it and put a quest counter on this enchantment.
// At the beginning of each end step, if there are three or more quest counters on this enchantment, you may put a Kraken, Leviathan, Octopus, or Serpent creature card from your hand onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_1, &objects_1);
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
if { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).get(0).copied().is_some_and(|candidate: ObjectId| q.chars(candidate).is(CardType::Creature)) } {
let payer_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_3 = cantrip_mechanics::effects::ask_may(g, payer_3, asker.source, Purpose::Optional)?; if paid_3 {
let objects_4 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_4: Vec<PlayerId> = objects_4.iter().map(|o| g.state().obj(*o).owner).collect(); owners_4.dedup(); for owner in owners_4 { let cards: Vec<ObjectId> = objects_4.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
hoisted_5.extend(objects_4.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_3 as i64) != 0) } {
let mut counters_6: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_6.push((object, CounterKind::Quest, amount)); } }
let had_6: Vec<_> = counters_6.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_6); let applied_6: Vec<ObjectId> = counters_6.iter().zip(had_6).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_7.extend(applied_6.clone());
}
g.end_sentence();
hoisted_8.extend(hoisted_5.clone());
hoisted_9.extend(hoisted_7.clone());
hoisted_10.extend(hoisted_5.clone());
hoisted_11.extend({ let q: &dyn Query = g; hoisted_5.clone() });
}
g.end_sentence();
hoisted_12.extend(hoisted_8.clone());
hoisted_13.extend(hoisted_9.clone());
hoisted_14.extend(hoisted_10.clone());
hoisted_15.extend({ let q: &dyn Query = g; hoisted_11.clone() });
}
g.end_sentence();
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Quest))) >= (3i64)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_20: Vec<ObjectId> = Vec::new();
let mut hoisted_19: Vec<ObjectId> = Vec::new();
let mut hoisted_18: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_16_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_16_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Kraken)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Leviathan)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Octopus)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Serpent))) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let selected_16_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_16 = { cantrip_mechanics::choose::select(g, selected_16_player, asker.source, Purpose::PutOntoBattlefield, selected_16_candidates, cantrip_mechanics::select::Selection::of(selected_16_count, &[]))? };
if !selected_16.is_empty() { 
let player_17 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_17 = ({ let q: &dyn Query = g; selected_16.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_17 = { let rows_17: Vec<(ObjectId, PlayerId, bool)> = objects_17.iter().map(|&o| (o, player_17, false)).collect(); let came_17 = g.put_all_onto_battlefield_as(&rows_17)?; came_17.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_18.extend(moved_17.clone());
hoisted_19.extend(selected_16.clone());
hoisted_20.extend({ let q: &dyn Query = g; objects_17.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Quest for Ula's Temple", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, you may look at the top card of your library. If it's a creature card, you may reveal it and put a quest counter on this enchantment.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each end step, if there are three or more quest counters on this enchantment, you may put a Kraken, Leviathan, Octopus, or Serpent creature card from your hand onto the battlefield.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
