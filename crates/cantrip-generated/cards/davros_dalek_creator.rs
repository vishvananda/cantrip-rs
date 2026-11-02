// Davros, Dalek Creator
// status: ready
// Menace
// At the beginning of your end step, create a 3/3 black Dalek artifact creature token with menace if an opponent lost 3 or more life this turn. Then each opponent who lost 3 or more life this turn faces a villainous choice — You draw a card, or that player discards a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dalek).name, token_words: token!(Dalek).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dalek]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta < 0 && Some(bound_0) == Some(*player) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()) >= (3i64)) } {
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
hoisted_2.extend(created_0.clone());
}
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta < 0 && Some(bound_0) == Some(*player) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()) >= (3i64)).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_3.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
let mut each_10: Vec<Vec<ObjectId>> = vec![Vec::new(); players_3.len()]; let mut all_10: Vec<ObjectId> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let decider_4 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let offered_4: Vec<u8> = [(0u8, true), (1u8, { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().any(|p: PlayerId| !q.state().zone(Zone::hand(p)).is_empty()) })].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_4 = item.ability.card; let picked_4 = if offered_4.len() > 1 { offered_4[cantrip_mechanics::choose::one_of(g, decider_4, asker.source, offered_4.iter().map(|&option| Label::Card(card_4, option)).collect())? as usize] } else { offered_4.first().copied().unwrap_or(u8::MAX) }; match picked_4 {
0 => {
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
}
1 => {
let amount_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_6: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_6 = Vec::new(); for player in players_6 { cards_6.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_6)?); } let discarded_6 = g.discard_together(&cards_6);
g.end_sentence();
hoisted_7.extend(discarded_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; discarded_6.clone() });
}
_ => {}
}
 each_9[index_3] = hoisted_7.clone(); all_9.extend(each_9[index_3].iter().copied());
each_10[index_3] = { let q: &dyn Query = g; hoisted_8.clone() }; all_10.extend(each_10[index_3].iter().copied());
   } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Davros, Dalek Creator", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien,Subtype::Scientist]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, create a 3/3 black Dalek artifact creature token with menace if an opponent lost 3 or more life this turn. Then each opponent who lost 3 or more life this turn faces a villainous choice — You draw a card, or that player discards a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["You draw a card", "That player discards a card"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
