// Multiple Choice
// status: planned
// requires: a choice of Player outside its body
// If X is 1, scry 1, then draw a card.
// If X is 2, you may choose a player. They return a creature they control to its owner's hand.
// If X is 3, create a 4/4 blue and red Elemental creature token.
// If X is 4 or more, do all of the above.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_15 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Elemental).name, token_words: token!(Elemental).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(10)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_29 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Elemental).name, token_words: token!(Elemental).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(10)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_33: Vec<ObjectId> = Vec::new();
let mut hoisted_32: Vec<ObjectId> = Vec::new();
let mut hoisted_31: Vec<ObjectId> = Vec::new();
let mut hoisted_30: Vec<ObjectId> = Vec::new();
let mut hoisted_27: Vec<ObjectId> = Vec::new();
let mut hoisted_26: Vec<ObjectId> = Vec::new();
let mut hoisted_25: Vec<ObjectId> = Vec::new();
let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (1i64) } {
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (2i64) } {
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_2 = cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)?; if paid_2 {
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let Some(chosen_3) = cantrip_mechanics::choose::player(g, player_3, asker.source, candidates_3, "Choose", 0)? else { return Ok(()) };
planned!("a-choice-of-player-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Player(chosen_3)););
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
let (carried_6_0,) = if let Some(present_6_0) = { let q: &dyn Query = g; planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked(item))) } { let selected_5_player = { let q: &dyn Query = g; present_6_0 }; let selected_5_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked(item))) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::PutIntoHand, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
(selected_5.clone(),) } else { Default::default() };
let objects_7 = { let q: &dyn Query = g; carried_6_0.clone() }; let moved_7 = g.return_to_hand(&objects_7);
g.end_sentence();
hoisted_8.extend(moved_7.clone());
hoisted_9.extend(carried_6_0.clone());
hoisted_10.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_11.extend(hoisted_8.clone());
hoisted_12.extend(hoisted_9.clone());
hoisted_13.extend({ let q: &dyn Query = g; hoisted_10.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (3i64) } {
let players_14 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_14 = Vec::new();
let count_14_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_14_0 = g.card_id(&token_15::CARD);
for player in players_14 {
tokens_14.extend(std::iter::repeat((def_14_0,player)).take(count_14_0 as usize));
} let words_14 = g.words(asker.source);
let created_14 = g.create_tokens_worded(&tokens_14, false, None, words_14)?;
g.end_sentence();
hoisted_16.extend(created_14.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) >= (4i64) } {
let amount_17 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_17 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_17, asker.source, amount_17)?;
g.end_sentence();
let amount_18 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_18 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_18 { g.draw(player, amount_18); }
g.end_sentence();
let payer_19 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_19 = cantrip_mechanics::effects::ask_may(g, payer_19, asker.source, Purpose::Optional)?; if paid_19 {
let player_20 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_20 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let Some(chosen_20) = cantrip_mechanics::choose::player(g, player_20, asker.source, candidates_20, "Choose", 0)? else { return Ok(()) };
planned!("a-choice-of-player-outside-its-body" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Player(chosen_20)););
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_19 as i64) != 0) } {
let (carried_23_0,) = if let Some(present_23_0) = { let q: &dyn Query = g; planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked(item))) } { let selected_22_player = { let q: &dyn Query = g; present_23_0 }; let selected_22_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("a choice of Player outside its body", None::<PlayerId> => q.state().chosen_player(q.linked(item))) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_22_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_22 = { cantrip_mechanics::choose::select(g, selected_22_player, asker.source, Purpose::PutIntoHand, selected_22_candidates, cantrip_mechanics::select::Selection::of(selected_22_count, &[]))? };
(selected_22.clone(),) } else { Default::default() };
let objects_24 = { let q: &dyn Query = g; carried_23_0.clone() }; let moved_24 = g.return_to_hand(&objects_24);
g.end_sentence();
hoisted_25.extend(moved_24.clone());
hoisted_26.extend(carried_23_0.clone());
hoisted_27.extend({ let q: &dyn Query = g; objects_24.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
let players_28 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_28 = Vec::new();
let count_28_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_28_0 = g.card_id(&token_29::CARD);
for player in players_28 {
tokens_28.extend(std::iter::repeat((def_28_0,player)).take(count_28_0 as usize));
} let words_28 = g.words(asker.source);
let created_28 = g.create_tokens_worded(&tokens_28, false, None, words_28)?;
g.end_sentence();
hoisted_30.extend(hoisted_25.clone());
hoisted_31.extend(hoisted_26.clone());
hoisted_32.extend(created_28.clone());
hoisted_33.extend({ let q: &dyn Query = g; hoisted_27.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Multiple Choice", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_15::CARD,&token_29::CARD];
