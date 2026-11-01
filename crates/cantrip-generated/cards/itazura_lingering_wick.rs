// Itazura, Lingering Wick
// status: planned
// requires: condition ChoseExtreme
// requires: numeric expression PlayerNumber ChosenNumber
// requires: operation ChooseValueWith
// requires: operation RevealChoices
// At the beginning of your upkeep, exile the top three cards of your library. Each opponent secretly chooses a number 0 or greater. Then those numbers are revealed. Choose an opponent with the highest number. Itazura deals that much damage to them, then they may cast a spell from among those cards without paying its mana cost. You put a card from among them that wasn't cast this way into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((3i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_3 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let chosen_2 = planned_value!("operation ChooseValueWith", 0u32 => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Number, ChoiceTerms { options: &["Comparison(GreaterOrEqual, Constant(0))"], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
 each_3[index_1] = i64::from(chosen_2);
   } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Number));
g.end_sentence();
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_5 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition ChoseExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.chose_extreme(asker, p, true)))).collect::<Vec<PlayerId>>() }; let Some(chosen_5) = cantrip_mechanics::choose::player(g, player_5, asker.source, candidates_5, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let amount_6 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| planned_value!("numeric expression PlayerNumber ChosenNumber", 0i64 => q.player_number(p, PlayerNumber::ChosenNumber))).max().unwrap_or(0)).clamp(0, 65535) as u16 }; let recipients_6 = { let q: &dyn Query = g; (Some(chosen_5).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_6 { let batch: Vec<(ObjectId, Target, u16)> = recipients_6.iter().map(|t| (source, *t, amount_6)).collect(); g.damage_together(&batch); }
g.end_sentence();
let caster_7 = { let q: &dyn Query = g; Some(chosen_5).expect("a reference that always names a player") }; let mut pool_7: Vec<ObjectId> = { let q: &dyn Query = g; (moved_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_7: u32 = 1; let mut played_7: Vec<ObjectId> = Vec::new();
while left_7 > 0 {
let mut open_7: Vec<ObjectId> = Vec::new(); for &c in &pool_7 { let ways = g.cast_ways_during_resolution_by(c, caster_7); if ways.iter().any(|w| true) { open_7.push(c); } }
let picked_7 = if open_7.is_empty() { None } else if open_7.len() > 1 || (true && (!false || pool_7.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_7, asker.source, Purpose::CastFree, pool_7.clone(), open_7.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_7, open_7[0], Purpose::CastFree)? { Some(open_7[0]) } else { None };
let Some(card_7) = picked_7 else { break; };
let ways_7: Vec<CastWay> = g.cast_ways_during_resolution_by(card_7, caster_7).into_iter().filter(|w| true).collect();
let done_7 = if ways_7.is_empty() { g.play_land_during_resolution(card_7)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_7, card_7, ways_7)?.map(|w| w.way); g.cast_during_resolution_as(card_7, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_7), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_7 { left_7 -= 1; played_7.push(g.state().obj(card_7).became.unwrap_or(card_7)); } else if 1i64 == 1 { break; } if done_7 || !(true && true) { pool_7.retain(|&c| c != card_7); }
}
g.end_sentence();
let selected_8_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; moved_0.clone() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::PutIntoHand, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
let objects_9 = { let q: &dyn Query = g; selected_8.clone() }; let moved_9 = g.return_to_hand(&objects_9);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Itazura, Lingering Wick", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of your upkeep, exile the top three cards of your library. Each opponent secretly chooses a number 0 or greater. Then those numbers are revealed. Choose an opponent with the highest number. Itazura deals that much damage to them, then they may cast a spell from among those cards without paying its mana cost. You put a card from among them that wasn't cast this way into your hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
