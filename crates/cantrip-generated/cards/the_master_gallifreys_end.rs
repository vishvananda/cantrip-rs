// The Master, Gallifrey's End
// status: planned
// requires: engine: a two-word subtype in a card's type line
// requires: villainous choice
// Make Them Pay — Whenever a nontoken artifact creature you control dies, you may exile it. If you do, choose an opponent with the most life among your opponents. That player faces a villainous choice — They lose 4 life, or you create a token that's a copy of that card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && (!(q.state().obj(id).token) && q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if ({ let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied() }).is_some() && cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let objects_1 = { let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
let paid_1 = moved_1.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_1, { let q: &dyn Query = g; (moved_1).clone() }, { let q: &dyn Query = g; (objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_2 = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| (Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) == ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| i64::from(q.state().player(p).life)).max().unwrap_or(0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } }; let Some(chosen_2) = cantrip_mechanics::choose::player(g, player_2, asker.source, candidates_2, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
planned!("villainous-choice" => cantrip_mechanics::choose::villainous(g, { let q: &dyn Query = g; Some(chosen_2).expect("a reference that always names a player") }, asker.source, &["Lose 4 life.", "Create tokens."])?);
let decider_4 = { let q: &dyn Query = g; Some(chosen_2).expect("a reference that always names a player") };
let offered_4: Vec<u8> = [(0u8, true), (1u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_4 = item.ability.card; let picked_4 = if offered_4.len() > 1 { offered_4[cantrip_mechanics::choose::one_of(g, decider_4, asker.source, offered_4.iter().map(|&option| Label::Card(card_4, option)).collect())? as usize] } else { offered_4.first().copied().unwrap_or(u8::MAX) }; match picked_4 {
0 => {
let amount_5 = { let q: &dyn Query = g; (4i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; Some(chosen_2).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.lose_life(player, amount_5); }
g.end_sentence();
}
1 => {
let original_6 = { let q: &dyn Query = g; (after_0.clone()).get(0).copied() }; let count_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_6 = { let q: &dyn Query = g; Some(chosen_2).expect("a reference that always names a player") }; let created_6 = match original_6 { Some(original) => g.create_token_copies(original, player_6, count_6)?, None => Vec::new() };
g.end_sentence();
hoisted_7.extend(created_6.clone());
}
_ => {}
}
g.end_sentence();
hoisted_8.extend(hoisted_7.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Master, Gallifrey's End", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[planned_value!("engine: a two-word subtype in a card's type line", Subtype::TimeLord => Subtype::TimeLord),Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Make Them Pay — Whenever a nontoken artifact creature you control dies, you may exile it. If you do, choose an opponent with the most life among your opponents. That player faces a villainous choice — They lose 4 life, or you create a token that's a copy of that card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["They lose 4 life", "You create a token that's a copy of that card"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
