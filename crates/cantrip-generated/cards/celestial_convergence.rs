// Celestial Convergence
// status: planned
// requires: engine: no effect draws the game
// This enchantment enters with seven omen counters on it.
// At the beginning of your upkeep, remove an omen counter from this enchantment. If there are no omen counters on this enchantment, the player with the highest life total wins the game. If two or more players are tied for highest life total, the game is a draw.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (7i64).max(0) as i16 }; entering.counters[CounterKind::Omen as usize] = entering.counters[CounterKind::Omen as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_1_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += i64::from(g.remove_counters(object, CounterKind::Omen, amount_1_0)); } });
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Omen))) == (0i64)) } {
if { let q: &dyn Query = g; (({ let mut all = q.state().apnap().collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| (Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) == ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| i64::from(q.state().player(p).life)).max().unwrap_or(0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) > (1i64) } {
planned!("engine-no-effect-draws-the-game" => g.draw_game(););
g.end_sentence();
} else {
if let Some(present_5_0) = { let q: &dyn Query = g; { let rows: Vec<(PlayerId, i64)> = (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| (p, i64::from(q.state().player(p).life))).collect(); let best = rows.iter().map(|r| r.1).max(); let mut top = rows.iter().filter(|r| Some(r.1) == best).map(|r| r.0); match (top.next(), top.next()) { (Some(p), None) => Some(p), _ => None } } } { { let player = { let q: &dyn Query = g; present_5_0 }; g.win_game(player); if g.state().is_over() { return Ok(()); }; }
 }
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Celestial Convergence", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your upkeep, remove an omen counter from this enchantment. If there are no omen counters on this enchantment, the player with the highest life total wins the game. If two or more players are tied for highest life total, the game is a draw.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
