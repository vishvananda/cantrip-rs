// The Second Doctor
// status: planned
// requires: a maximum hand size of other players
// requires: duration DuringNextTurn
// requires: player rule for a while Prohibit AttackInto
// Players have no maximum hand size.
// How Civil of You — At the beginning of your end step, each player may draw a card. Each opponent who does can't attack you or permanents you control during their next turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_3 = vec![Default::default(); players_0.len()];
let mut done_total_3: i64 = 0;
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if cantrip_mechanics::effects::ask_may(g, bound_0, asker.source, Purpose::Optional)? { guard_0_0[index_0] = true;
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
let mut done_1: i64 = 0; let mut drawn_1: Vec<ObjectId> = Vec::new(); for player in players_1 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_1); done_1 += g.history().drawn(player) as i64 - before; drawn_1.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
 each_2[index_0] = drawn_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_3[index_0] = done_1;
done_total_3 += done_1;
  }
 } 
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_0.iter().position(|&next| next == p)).is_some_and(|at| (each_3[at] as i64) != 0)).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
g.one_instruction(&mut |g| { for (index_4, bound_0) in players_4.iter().copied().enumerate() { planned!("player-rule-for-a-while-prohibit-attack-into" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Cant(Prohibition::AttackInto({ let q: &dyn Query = g; (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }))), planned_value!("duration DuringNextTurn", Duration::Indefinite => Duration::DuringNextTurn(Bound(Binding(0))))); } });
planned!("player-rule-for-a-while-prohibit-attack-into" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Cant(Prohibition::AttackInto({ let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() }))), planned_value!("duration DuringNextTurn", Duration::Indefinite => Duration::DuringNextTurn(Bound(Binding(0))))); } });
    } Ok(()) })?;
g.end_sentence();
 Ok(()) }
planned_item!("a-maximum-hand-size-of-other-players" => hand_size_rules: HandSizeRule { applies: |q, asker, player| true, size: HandSize::Unlimited });

pub static CARD: CardDef = CardDef { name: "The Second Doctor", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::TimeLord,Subtype::Doctor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "How Civil of You — At the beginning of your end step, each player may draw a card. Each opponent who does can't attack you or permanents you control during their next turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
