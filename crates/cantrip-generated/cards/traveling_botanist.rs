// Traveling Botanist
// status: ready
// Whenever this creature becomes tapped, look at the top card of your library. If it's a land card, you may reveal it and put it into your hand. If you don't put the card into your hand, you may put it into your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Tapped { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
g.end_sentence();
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let looked_1 = cantrip_mechanics::top_of_library(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, 1); let offered_1 = { let q: &dyn Query = g; looked_1.iter().copied().any(|id| q.chars(id).is(CardType::Land)) };
let paid_1 = offered_1 && cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)?; if paid_1 {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let top_2 = cantrip_mechanics::top_of_library(g, player_2, 1);
g.reveal(player_2, &top_2);
let matching_2: Vec<ObjectId> = { let q: &dyn Query = g; top_2.iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect() };
let objects_3 = { let q: &dyn Query = g; matching_2.clone() }; let moved_3 = g.return_to_hand_revealed(&objects_3);
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_1 as i64) != 0)) } {
let decider_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !({ let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }).is_empty() && cantrip_mechanics::effects::ask_may(g, decider_6, asker.source, Purpose::Optional)? {
let objects_7 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_7 = { let moves: Vec<(ObjectId, Zone)> = objects_7.iter().map(|&o| (o, Zone::graveyard(g.state().obj(o).owner))).collect(); let moved = g.move_together(&moves, MoveReason::Other); moved.into_iter().filter(|&o| g.state().obj(o).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_8.extend(moved_7.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_10.extend(hoisted_8.clone());
hoisted_11.extend({ let q: &dyn Query = g; hoisted_9.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Traveling Botanist", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dog,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), ability_0_event), ability_0, "Whenever this creature becomes tapped, look at the top card of your library. If it's a land card, you may reveal it and put it into your hand. If you don't put the card into your hand, you may put it into your graveyard.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
