// The Temporal Anchor
// status: planned
// requires: trigger event Player BottomsWhileScrying
// At the beginning of your upkeep, scry 2.
// Whenever you choose to put one or more cards on the bottom of your library while scrying, exile that many cards from the bottom of your library.
// During your turn, you may play cards exiled with The Temporal Anchor.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; { let mut end = (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().rev().take((planned_value!("depends on an instruction with a gap", 0i64 => q.event_total(item))).max(0) as usize).collect::<Vec<ObjectId>>(); end.reverse(); end } }; let moved_1 = { let by = g.linked(item); g.exile_with(&objects_1, by, false) };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_0_allows(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> bool { player == asker.controller && (Some(asker.controller) == Some(q.state().turn.active)) && q.exiled_with(q.linked_to(asker)).contains(&card) }
planned_item!("trigger-event-player-bottoms-while-scrying" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BottomedWhileScrying { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "The Temporal Anchor", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, scry 2.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player BottomsWhileScrying", EventMask(0) => EventMask(EventMask::BOTTOMED.0)), ability_1_event), ability_1, "Whenever you choose to put one or more cards on the bottom of your library while scrying, exile that many cards from the bottom of your library.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, exile_casts: &[ExileCast { text: "Play from exile", allows: ability_2_0_0_allows, additional: Cost::NONE, any_type: false, lands: true, life_for_mana_value: false }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
