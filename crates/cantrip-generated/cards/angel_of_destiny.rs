// Angel of Destiny
// status: ready
// Flying, double strike
// Whenever a creature you control deals combat damage to a player, you and that player each gain that much life.
// At the beginning of your end step, if you have at least 15 life more than your starting life total, each player this creature attacked this turn loses the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.data[0])).min(i32::MAX.into()) as i32 }; let players_0 = { let q: &dyn Query = g; { let mut all: Vec<PlayerId> = Vec::new(); for p in Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } for p in Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() { if !all.contains(&p) { all.push(p); } } all } };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) >= ((15i64 + Some(asker.controller).map_or(0, |player: PlayerId| i64::from(q.state().player(player).starting_life)))) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if Some(asker.source) == Some(id) && match *defender { Target::Player(p) => Some(bound_0) == Some(p), _ => false } { return Some(Fired { subject, data }); } }  None }).next().is_some()).collect::<Vec<PlayerId>>() };
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Angel of Destiny", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::DoubleStrike]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever a creature you control deals combat damage to a player, you and that player each gain that much life.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_3_event), ability_3, "At the beginning of your end step, if you have at least 15 life more than your starting life total, each player this creature attacked this turn loses the game.").targets(&[]).condition(ability_3_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
