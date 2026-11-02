// Struggle for Project Purity
// status: planned
// requires: amount of an event its trigger does not carry
// requires: rad counters
// As this enchantment enters, choose Brotherhood or Enclave.
// • Brotherhood — At the beginning of your upkeep, each opponent draws a card. You draw a card for each card drawn this way.
// • Enclave — Whenever a player attacks you with one or more creatures, that player gets twice that many rad counters.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..2)?; let chosen_0: u8 = entering.chosen_option.map_or(0, |chosen| chosen.index);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) && cantrip_mechanics::choose::chosen_option(q, asker, &CARD) == Some(0) { return Some(Fired { subject, data }); } }
 None }
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let drawing_1: Vec<PlayerId> = players_1.into_iter().collect(); let players_1: Vec<PlayerId> = g.state().apnap().filter(|p| drawing_1.contains(p)).collect();
let mut done_1: i64 = 0; let mut drawn_1: Vec<ObjectId> = Vec::new(); for player in players_1 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_1); done_1 += g.history().drawn(player) as i64 - before; drawn_1.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; ((done_1 as i64)).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if q.chars(id).is(CardType::Creature) && true && match *defender { Target::Player(p) => Some(asker.controller) == Some(p), _ => false } && cantrip_mechanics::choose::chosen_option(q, asker, &CARD) == Some(1) { return Some(Fired { subject, data }); } }
 None }
fn ability_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
planned!("rad-counters" => let amount = { let q: &dyn Query = g; ((2i64 * planned_value!("amount of an event its trigger does not carry", 0i64 => q.event_total(item)))).clamp(0, 4294967295) as u32 }; for player in { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::rad::give(g, player, amount); });
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Struggle for Project Purity", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_0_event), ability_1_0, "As this enchantment enters, choose Brotherhood or Enclave. Brotherhood — At the beginning of your upkeep, each opponent draws a card. You draw a card for each card drawn this way; or Enclave — Whenever a player attacks you with one or more creatures, that player gets twice that many rad counters.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_0_event), ability_2_0, "As this enchantment enters, choose Brotherhood or Enclave. Brotherhood — At the beginning of your upkeep, each opponent draws a card. You draw a card for each card drawn this way; or Enclave — Whenever a player attacks you with one or more creatures, that player gets twice that many rad counters.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["Brotherhood", "Enclave"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
