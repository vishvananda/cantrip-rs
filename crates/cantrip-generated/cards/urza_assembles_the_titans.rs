// Urza Assembles the Titans
// status: planned
// requires: engine: no ReadAhead
// requires: player rule for a while ActivationGrant
// Read ahead (Choose a chapter and start with that many lore counters. Add one after your draw step. Skipped chapters don't trigger. Sacrifice after III.)
// I — Scry 4, then you may reveal the top card of your library. If a planeswalker card is revealed this way, put it into your hand.
// II — You may put a planeswalker card with mana value 6 or less from your hand onto the battlefield.
// III — You may activate the loyalty abilities of planeswalkers you control twice this turn rather than only once.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_1 = cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)?; if paid_1 {
let objects_2 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let mut owners_2: Vec<PlayerId> = objects_2.iter().map(|o| g.state().obj(*o).owner).collect(); owners_2.dedup(); for owner in owners_2 { let cards: Vec<ObjectId> = objects_2.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
hoisted_3.extend(objects_2.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
if { let q: &dyn Query = g; ((((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.chars(id).is(CardType::Planeswalker)).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) } {
let objects_4 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_4 = g.return_to_hand_revealed(&objects_4);
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_7.extend(hoisted_5.clone());
hoisted_8.extend({ let q: &dyn Query = g; hoisted_6.clone() });
}
g.end_sentence();
 Ok(()) }
fn ability_1_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_9_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_9_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Planeswalker) && (i64::from(q.mana_value(id))) <= (6i64))).collect::<Vec<ObjectId>>() }; let selected_9_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_9 = { cantrip_mechanics::choose::select(g, selected_9_player, asker.source, Purpose::PutOntoBattlefield, selected_9_candidates, cantrip_mechanics::select::Selection::of(selected_9_count, &[]))? };
if !selected_9.is_empty() { 
let player_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_10 = ({ let q: &dyn Query = g; selected_9.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_10 = { let rows_10: Vec<(ObjectId, PlayerId, bool)> = objects_10.iter().map(|&o| (o, player_10, false)).collect(); let came_10 = g.put_all_onto_battlefield_as(&rows_10)?; came_10.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_11.extend(moved_10.clone());
hoisted_12.extend(selected_9.clone());
hoisted_13.extend({ let q: &dyn Query = g; objects_10.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
fn ability_1_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-activation-grant" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::ActivationGrant { abilities: |q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| (def.is_some_and(|d| d.is_loyalty_ability()) && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && (q.chars(permanent).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(permanent))))), grant: ActivationGrant::AdditionalTimes(1i64) }), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-read-ahead" => enters: cantrip_mechanics::read_ahead::enters);

pub static CARD: CardDef = CardDef { name: "Urza Assembles the Titans", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_1_0, "I — Scry 4, then you may reveal the top card of your library. If a planeswalker card is revealed this way, put it into your hand.").targets(&[]),chapter(2, ability_1_1, "II — You may put a planeswalker card with mana value 6 or less from your hand onto the battlefield.").targets(&[]),chapter(3, ability_1_2, "III — You may activate the loyalty abilities of planeswalkers you control twice this turn rather than only once.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
