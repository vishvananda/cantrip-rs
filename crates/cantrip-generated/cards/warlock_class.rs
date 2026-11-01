// Warlock Class
// status: ready
// (Gain the next level as a sorcery to add its ability.)
// At the beginning of your end step, if a creature died this turn, each opponent loses 1 life.
// {1}{B}: Level 2
// When this Class becomes level 2, look at the top three cards of your library. Put one of them into your hand and the rest into your graveyard.
// {6}{B}: Level 3
// At the beginning of your end step, each opponent loses life equal to the life they lost this turn. (Damage causes loss of life.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && q.chars(id).is(CardType::Creature) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_2_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ClassLevel { object, level } = event { let id = *object; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) && *level == (2i64).max(0) as u8 && q.state().obj(asker.source).class_level >= 2 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_2_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let depth_1 = { let q: &dyn Query = g; (3i64).max(0) as usize }; cantrip_mechanics::library::look_at_top_take_one_rest_to_graveyard(g, player_1, asker.source, depth_1)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_3_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) && q.state().obj(asker.source).class_level >= 3 { return Some(Fired { subject, data }); } }
 None }
fn ability_1_level_3_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
 for (index_2, bound_9000) in players_2.iter().copied().enumerate() { let amount_3 = { let q: &dyn Query = g; (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta < 0 && Some(bound_9000) == Some(*player) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()).clamp(0, 2147483647) as i32 }; let players_3 = { let q: &dyn Query = g; Some(bound_9000).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Warlock Class", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your end step, if a creature died this turn, each opponent loses 1 life.").targets(&[]).condition(ability_0_if),cantrip_mechanics::class::level_two("{1}{B}", "{1}{B}: Level 2"),triggered(when_asked(EventMask(EventMask::CLASS_LEVEL.0), ability_1_level_2_0_event), ability_1_level_2_0, "When this Class becomes that level,   Return the chosen permanents to their owners' hands. Return the chosen permanents to their owners' hands.").targets(&[]),cantrip_mechanics::class::level_three("{6}{B}", "{6}{B}: Level 3"),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_level_3_0_event), ability_1_level_3_0, "At the beginning of this step, Lose life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
