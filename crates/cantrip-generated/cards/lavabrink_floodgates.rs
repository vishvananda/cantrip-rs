// Lavabrink Floodgates
// status: ready
// {T}: Add {R}{R}.
// At the beginning of each player's upkeep, that player may put a doom counter on this artifact or remove a doom counter from it. Then if it has three or more doom counters on it, sacrifice this artifact. When you do, it deals 6 damage to each creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::R, mana_0_0);}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn reflexive_9(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let amount_10 = { let q: &dyn Query = g; (6i64).max(0) as u16 }; let recipients_10 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }; if let Some(source) = source_10 { let batch: Vec<(ObjectId, Target, u16)> = recipients_10.iter().map(|t| (source, *t, amount_10)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let decider_2 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") };
let offered_2: Vec<u8> = [(0u8, true), (1u8, { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| q.state().obj(o).counters(CounterKind::Doom) > 0) })].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_2 = item.ability.card; let picked_2 = if offered_2.len() > 1 { offered_2[cantrip_mechanics::choose::one_of(g, decider_2, asker.source, offered_2.iter().map(|&option| Label::Card(card_2, option)).collect())? as usize] } else { offered_2.first().copied().unwrap_or(u8::MAX) }; match picked_2 {
0 => {
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::Doom, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_4.extend(applied_3.clone());
}
1 => {
let mut removed_5: i64 = 0;
let objects_5_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_5_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_5_0 { removed_5 += i64::from(g.remove_counters(object, CounterKind::Doom, amount_5_0)); } });
g.end_sentence();
}
_ => {}
}
g.end_sentence();
hoisted_6.extend(hoisted_4.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Doom))) >= (3i64)) } {
let objects_7 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_7: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_7 { let controller = g.controller(object); if players_7.contains(&controller) && g.can_sacrifice(controller, object) { picks_7.push((controller, object)); } }
let sacrificed_7 = g.sacrifice_each(&picks_7);
if !sacrificed_7.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_11.extend(sacrificed_7.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Lavabrink Floodgates", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 2, 0, 0]), ..add_by(ability_0_mana) }, "{T}: Add {R}{R}."),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each player's upkeep, that player may put a doom counter on this artifact or remove a doom counter from it. Then if it has three or more doom counters on it, sacrifice this artifact. When you do, it deals 6 damage to each creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_9, "When you do, it deals 6 damage to each creature.").targets(&[])], enters: None, modal: None, enchant: None, labels: &["Put a doom counter on this artifact", "Remove a doom counter from it"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
