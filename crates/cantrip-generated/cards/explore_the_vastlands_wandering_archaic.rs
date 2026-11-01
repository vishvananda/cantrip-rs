// Explore the Vastlands // Wandering Archaic
// status: ready
// Each player looks at the top five cards of their library and may reveal a land card and/or an instant or sorcery card from among them. Each player puts the cards they revealed this way into their hand and the rest on the bottom of their library in a random order. Each player gains 3 life.
// Whenever an opponent casts an instant or sorcery spell, they may pay {2}. If they don't, you may copy that spell. You may choose new targets for the copy.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let copier_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_2: Option<ObjectId> = ({ let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()) }).first().copied();
if let Some(spell) = spell_2 { g.copy_spell(spell, copier_2, true)?; }
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Wandering Archaic", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever an opponent casts an instant or sorcery spell, they may pay {2}. If they don't, you may copy that spell. You may choose new targets for the copy.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let objects_1 = { let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((5i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_1, &objects_1);
 each_2[index_0] = objects_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_3_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((each_2[index_0].clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::ChooseAsEnters, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
 each_4[index_0] = selected_3.clone(); all_4.extend(each_4[index_0].iter().copied());
   } 
g.end_sentence();
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_5_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_5_candidates = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in each_4[index_0].clone() { if !all.contains(&id) { all.push(id); } } all }; ((each_2[index_0].clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::ChooseAsEnters, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
 each_6[index_0] = selected_5.clone(); all_6.extend(each_6[index_0].iter().copied());
   } 
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in all_4.clone() { if !all.contains(&id) { all.push(id); } } for id in all_6.clone() { if !all.contains(&id) { all.push(id); } } all } }; let mut owners_7: Vec<PlayerId> = objects_7.iter().map(|o| g.state().obj(*o).owner).collect(); owners_7.dedup(); for owner in owners_7 { let cards: Vec<ObjectId> = objects_7.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in all_4.clone() { if !all.contains(&id) { all.push(id); } } for id in all_6.clone() { if !all.contains(&id) { all.push(id); } } all } }; let moved_8 = g.return_to_hand_revealed(&objects_8);
g.end_sentence();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
let mut each_11: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_11: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let library_9 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let cards_9: Vec<ObjectId> = { let q: &dyn Query = g; ({ let without: Vec<ObjectId> = { let mut all: Vec<ObjectId> = Vec::new(); for id in objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all }; (each_2[index_0].clone()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() }).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_9)).collect(); let placed_9 = cards_9.clone();
g.bottom_in_random_order(library_9, &cards_9);
 each_10[index_0] = placed_9.clone(); all_10.extend(each_10[index_0].iter().copied());
each_11[index_0] = { let q: &dyn Query = g; placed_9.clone() }; all_11.extend(each_11[index_0].iter().copied());
   } 
g.end_sentence();
let amount_12 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_12 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_12 { g.gain_life(player, amount_12); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Explore the Vastlands", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
