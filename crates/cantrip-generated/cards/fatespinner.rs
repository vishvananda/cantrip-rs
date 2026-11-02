// Fatespinner
// status: planned
// requires: player rule for a while Skip
// At the beginning of each opponent's upkeep, that player chooses draw step, main phase, or combat phase. The player skips each instance of the chosen step or phase this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; item.subject.player().expect("a reference that always names a player") };
let offered_0: Vec<u8> = [(0u8, true), (1u8, true), (2u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_0 = item.ability.card; let picked_0 = if offered_0.len() > 1 { offered_0[cantrip_mechanics::choose::one_of(g, decider_0, asker.source, offered_0.iter().map(|&option| Label::Card(card_0, option)).collect())? as usize] } else { offered_0.first().copied().unwrap_or(u8::MAX) }; match picked_0 {
0 => {
planned!("player-rule-for-a-while-skip" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; item.subject.player().into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Skip(TurnSection::Draw)), Duration::EndOfTurn); } });
g.end_sentence();
}
1 => {
planned!("player-rule-for-a-while-skip" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; item.subject.player().into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Skip(TurnSection::Main)), Duration::EndOfTurn); } });
g.end_sentence();
}
2 => {
planned!("player-rule-for-a-while-skip" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; item.subject.player().into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::Skip(TurnSection::Combat)), Duration::EndOfTurn); } });
g.end_sentence();
}
_ => {}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Fatespinner", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of each opponent's upkeep, that player chooses draw step, main phase, or combat phase. The player skips each instance of the chosen step or phase this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Draw step", "Main phase", "Combat phase"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
