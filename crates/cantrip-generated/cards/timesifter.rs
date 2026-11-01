// Timesifter
// status: planned
// requires: condition HadExtreme
// requires: keyword action SetRepeatablePlayers
// requires: player collection Scoped
// At the beginning of each upkeep, each player exiles the top card of their library. The player who exiled the card with the greatest mana value takes an extra turn after this one. If two or more players' cards are tied for greatest, the tied players repeat this process until the tie is broken.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
planned!("keyword-action-set-repeatable-players" => cantrip_mechanics::players::set_repeatable(g, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, asker.source)?;);
g.end_sentence();
loop { let mut again_1 = false;
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|p: PlayerId| (planned_value!("player collection Scoped", Vec::<PlayerId>::new() => q.players_in_scope(asker))).contains(&p))).collect::<Vec<PlayerId>>() };
let mut gathered_3: Vec<ObjectId> = Vec::new();
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { gathered_3.extend({ let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() });
    } 
let objects_5 = { let q: &dyn Query = g; gathered_3.clone() }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
if { let q: &dyn Query = g; (({ let mut all = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|p: PlayerId| (planned_value!("player collection Scoped", Vec::<PlayerId>::new() => q.players_in_scope(asker))).contains(&p))).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition HadExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.had_extreme(item, p, EarlierObjects::Exiled, |id: ObjectId| true, true)))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64) >= (2i64) } {
planned!("keyword-action-set-repeatable-players" => cantrip_mechanics::players::set_repeatable(g, { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition HadExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.had_extreme(item, p, EarlierObjects::Exiled, |id: ObjectId| true, true)))).collect::<Vec<PlayerId>>() }, asker.source)?;);
g.end_sentence();
again_1 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_8.extend(moved_5.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
if !again_1 { break; } }
g.end_sentence();
let mut players_10: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition HadExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.had_extreme(item, p, EarlierObjects::Exiled, |id: ObjectId| true, true)))).collect::<Vec<PlayerId>>() };
 for (index_10, bound_0) in players_10.iter().copied().enumerate() { { let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let times = 1u32; g.add_extra_turns(player, times); }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Timesifter", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of each upkeep, each player exiles the top card of their library. The player who exiled the card with the greatest mana value takes an extra turn after this one. If two or more players' cards are tied for greatest, the tied players repeat this process until the tie is broken.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
