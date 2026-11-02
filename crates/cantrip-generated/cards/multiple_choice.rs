// Multiple Choice
// status: ready
// If X is 1, scry 1, then draw a card.
// If X is 2, you may choose a player. They return a creature they control to its owner's hand.
// If X is 3, create a 4/4 blue and red Elemental creature token.
// If X is 4 or more, do all of the above.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_13 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Elemental).name, token_words: token!(Elemental).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(10)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_25 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Elemental).name, token_words: token!(Elemental).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(10)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut chosen_player_kept = None;
let mut hoisted_29: Vec<ObjectId> = Vec::new();
let mut hoisted_28: Vec<ObjectId> = Vec::new();
let mut hoisted_27: Vec<ObjectId> = Vec::new();
let mut hoisted_26: Vec<ObjectId> = Vec::new();
let mut hoisted_23: Vec<ObjectId> = Vec::new();
let mut hoisted_22: Vec<ObjectId> = Vec::new();
let mut hoisted_21: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
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
g.end_sentence();
chosen_player_kept = Some(chosen_3);
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
let selected_4_player = { let q: &dyn Query = g; chosen_player_kept.expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && chosen_player_kept == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::PutIntoHand, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
let objects_5 = { let q: &dyn Query = g; selected_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
g.end_sentence();
hoisted_6.extend(moved_5.clone());
hoisted_7.extend(selected_4.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_9.extend(hoisted_6.clone());
hoisted_10.extend(hoisted_7.clone());
hoisted_11.extend({ let q: &dyn Query = g; hoisted_8.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) == (3i64) } {
let players_12 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_12 = Vec::new();
let count_12_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_12_0 = g.card_id(&token_13::CARD);
for player in players_12 {
tokens_12.extend(std::iter::repeat((def_12_0,player)).take(count_12_0 as usize));
} let words_12 = g.words(asker.source);
let created_12 = g.create_tokens_worded(&tokens_12, false, None, words_12)?;
g.end_sentence();
hoisted_14.extend(created_12.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))) >= (4i64) } {
let amount_15 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_15 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_15, asker.source, amount_15)?;
g.end_sentence();
let amount_16 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_16 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_16 { g.draw(player, amount_16); }
g.end_sentence();
let payer_17 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_17 = cantrip_mechanics::effects::ask_may(g, payer_17, asker.source, Purpose::Optional)?; if paid_17 {
let player_18 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let candidates_18 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let Some(chosen_18) = cantrip_mechanics::choose::player(g, player_18, asker.source, candidates_18, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
chosen_player_kept = Some(chosen_18);
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_17 as i64) != 0) } {
let selected_19_player = { let q: &dyn Query = g; chosen_player_kept.expect("a reference that always names a player") }; let selected_19_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && chosen_player_kept == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_19_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_19 = { cantrip_mechanics::choose::select(g, selected_19_player, asker.source, Purpose::PutIntoHand, selected_19_candidates, cantrip_mechanics::select::Selection::of(selected_19_count, &[]))? };
let objects_20 = { let q: &dyn Query = g; selected_19.clone() }; let moved_20 = g.return_to_hand(&objects_20);
g.end_sentence();
hoisted_21.extend(moved_20.clone());
hoisted_22.extend(selected_19.clone());
hoisted_23.extend({ let q: &dyn Query = g; objects_20.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
let players_24 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_24 = Vec::new();
let count_24_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_24_0 = g.card_id(&token_25::CARD);
for player in players_24 {
tokens_24.extend(std::iter::repeat((def_24_0,player)).take(count_24_0 as usize));
} let words_24 = g.words(asker.source);
let created_24 = g.create_tokens_worded(&tokens_24, false, None, words_24)?;
g.end_sentence();
hoisted_26.extend(hoisted_21.clone());
hoisted_27.extend(hoisted_22.clone());
hoisted_28.extend(created_24.clone());
hoisted_29.extend({ let q: &dyn Query = g; hoisted_23.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Multiple Choice", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_13::CARD,&token_25::CARD];
