// Sycorax Commander
// status: planned
// requires: villainous choice
// First strike, haste
// Sanctified Rules of Combat — When this creature enters, each opponent faces a villainous choice — That opponent discards all the cards in their hand, then draws that many cards minus one, or this creature deals damage to that player equal to the number of cards in their hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { planned!("villainous-choice" => cantrip_mechanics::choose::villainous(g, { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }, asker.source, &[" Draw cards.", ""])?);
let decider_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let offered_2: Vec<u8> = [(0u8, { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().any(|p: PlayerId| !q.state().zone(Zone::hand(p)).is_empty()) }), (1u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_2 = item.ability.card; let picked_2 = if offered_2.len() > 1 { offered_2[cantrip_mechanics::choose::one_of(g, decider_2, asker.source, offered_2.iter().map(|&option| Label::Card(card_2, option)).collect())? as usize] } else { offered_2.first().copied().unwrap_or(u8::MAX) }; match picked_2 {
0 => {
let players_3 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_3 = g.discard_together(&cards_3);
let amount_4 = { let q: &dyn Query = g; (((discarded_3.iter().copied().filter(|&o| Some(q.state().obj(o).owner) == Some(bound_0)).collect::<Vec<ObjectId>>()).len() as i64 - 1i64)).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
hoisted_5.extend(discarded_3.clone());
hoisted_6.extend({ let q: &dyn Query = g; discarded_3.clone() });
}
1 => {
let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let amount_7 = { let q: &dyn Query = g; ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 65535) as u16 }; let recipients_7 = { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_7 { let batch: Vec<(ObjectId, Target, u16)> = recipients_7.iter().map(|t| (source, *t, amount_7)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
_ => {}
}
 each_8[index_0] = hoisted_5.clone(); all_8.extend(each_8[index_0].iter().copied());
each_9[index_0] = { let q: &dyn Query = g; hoisted_6.clone() }; all_9.extend(each_9[index_0].iter().copied());
   } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sycorax Commander", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Sanctified Rules of Combat — When this creature enters, each opponent faces a villainous choice — That opponent discards all the cards in their hand, then draws that many cards minus one, or this creature deals damage to that player equal to the number of cards in their hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["That opponent discards all the cards in their hand, then draws that many cards minus one", "This creature deals damage to that player equal to the number of cards in their hand"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
