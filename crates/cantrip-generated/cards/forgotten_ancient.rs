// Forgotten Ancient
// status: ready
// Whenever a player casts a spell, you may put a +1/+1 counter on this creature.
// At the beginning of your upkeep, you may move any number of +1/+1 counters from this creature onto other creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if true && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::PlusOne, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_2.extend(applied_1.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_3, asker.source, Purpose::Optional)? {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let from_4: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let to_4: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let mut moved_4: i64 = 0; loop { let holders: Vec<ObjectId> = from_4.iter().copied().filter(|&o| to_4.iter().any(|&t| t != o) && g.state().obj(o).counters(CounterKind::PlusOne) > 0).collect(); let holder = match holders.as_slice() { [] => None, [one] => Some(*one), _ => cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::Choose, holders.clone(), 1, true)?.first().copied() }; let Some(holder) = holder else { break; }; let Some(kind) = Some(CounterKind::PlusOne) else { break; }; let hosts: Vec<ObjectId> = to_4.iter().copied().filter(|&t| t != holder).collect(); let host = match hosts.as_slice() { [one] => Some(*one), _ => cantrip_mechanics::choose::cards(g, player_4, asker.source, Purpose::PutCounters(kind, 1), hosts.clone(), 1, true)?.first().copied() }; let Some(host) = host else { break; }; let most = g.state().obj(holder).counters(kind).max(0) as u32; let many = cantrip_mechanics::choose::number(g, player_4, asker.source, 0, most, "Move how many counters")?; if many == 0 { break; } let done = g.move_counters(holder, host, kind, many as i16); moved_4 += i64::from(done); if done == 0 { break; } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Forgotten Ancient", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever a player casts a spell, you may put a +1/+1 counter on this creature.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, you may move any number of +1/+1 counters from this creature onto other creatures.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
