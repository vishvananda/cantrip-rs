// Priority Boarding
// status: ready
// Whenever you roll a die, you may reveal the top card of your library. Do this only once each turn. Whenever you reveal a card with mana value less than the result this way, you may exile it. If you do, you may play it this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::DieRolled { player, result, .. } = event { let subject = Target::Player(*player); let data = [(*result).max(0) as u32, 0]; if Some(asker.controller) == Some(*player) && !q.history().this_turn().iter().any(|r| matches!(r.event, Event::Done { ability, player, .. } if ability == AbilityRef { card: q.chars(asker.source).card, index: 200 } && player == asker.controller)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !g.history().done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1) && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? { g.note_done(asker.source, AbilityRef { card: g.chars(asker.source).card, index: 200 }, decider_1);
let objects_2 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
if { let q: &dyn Query = g; (((objects_2.clone()).into_iter().filter(|&id| (i64::from(q.mana_value(id))) < (i64::from(item.data[0]))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let payer_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_3, after_3, before_3, picked_3): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let selected_5_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_5_candidates = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::Exile, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
let objects_4 = { let q: &dyn Query = g; selected_5.clone() }; let moved_4 = g.exile_all(&objects_4);
let paid_4 = moved_4.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_4, { let q: &dyn Query = g; (moved_4).clone() }, { let q: &dyn Query = g; (objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_5).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_3 as i64) != 0) } {
cantrip_api::instruction(g, |g| { let cards_6_0 = { let q: &dyn Query = g; (after_3.clone()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_6_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_6_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_6_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
}
g.end_sentence();
hoisted_7.extend(after_3.clone());
hoisted_8.extend(picked_3.clone());
hoisted_9.extend({ let q: &dyn Query = g; before_3.clone() });
}
g.end_sentence();
hoisted_10.extend(hoisted_7.clone());
hoisted_11.extend(objects_2.clone());
hoisted_12.extend(hoisted_8.clone());
hoisted_13.extend({ let q: &dyn Query = g; hoisted_9.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Priority Boarding", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DIE_ROLLED.0), ability_0_event), ability_0, "Whenever you roll a die, you may reveal the top card of your library. Do this only once each turn. Whenever you reveal a card with mana value less than the result this way, you may exile it. If you do, you may play it this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
