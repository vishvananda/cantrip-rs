// Triskaidekaphobia
// status: ready
// At the beginning of your upkeep, choose one —
// • Each player with exactly 13 life loses the game, then each player gains 1 life.
// • Each player with exactly 13 life loses the game, then each player loses 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) == (13i64)).collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
    } 
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_2 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_2 { g.gain_life(player, amount_2); }
g.end_sentence();
 Ok(()) }
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) == (13i64)).collect::<Vec<PlayerId>>() };
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
    } 
g.end_sentence();
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_5 { g.lose_life(player, amount_5); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = modes(&[mode(ability_0_mode_0, "Each player with exactly 13 life loses the game, then each player gains 1 life.").targets(&[]),mode(ability_0_mode_1, "Each player with exactly 13 life loses the game, then each player loses 1 life.").targets(&[])]).choose(1, 1);

pub static CARD: CardDef = CardDef { name: "Triskaidekaphobia", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), &ABILITY_0_MODES, "At the beginning of your upkeep, choose one — Each player with exactly 13 life loses the game, then each player gains 1 life; or Each player with exactly 13 life loses the game, then each player loses 1 life.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
