// Tymaret Calls the Dead
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I, II — Mill three cards. Then you may exile a creature or enchantment card from your graveyard. If you do, create a 2/2 black Zombie creature token.
// III — You gain X life and scry X, where X is the number of Zombies you control.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Zombie).name, token_words: token!(Zombie).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_1, after_1, before_1, picked_1): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let selected_3_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_3_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let selected_3_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_3 = { cantrip_mechanics::choose::select(g, selected_3_player, asker.source, Purpose::Exile, selected_3_candidates, cantrip_mechanics::select::Selection::of(selected_3_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_3.clone() }; let moved_2 = g.exile_all(&objects_2);
let paid_2 = moved_2.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_2, { let q: &dyn Query = g; (moved_2).clone() }, { let q: &dyn Query = g; (objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_3).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_4 = Vec::new();
let count_4_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_4_0 = g.card_id(&token_5::CARD);
for player in players_4 {
tokens_4.extend(std::iter::repeat((def_4_0,player)).take(count_4_0 as usize));
} let words_4 = g.words(asker.source);
let created_4 = g.create_tokens_worded(&tokens_4, false, None, words_4)?;
g.end_sentence();
hoisted_6.extend(created_4.clone());
}
g.end_sentence();
 Ok(()) }
pub mod token_12 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Zombie).name, token_words: token!(Zombie).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_7 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_7: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_7)).collect(); let milled_7 = g.mill_together(&mills_7);
g.end_sentence();
let payer_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_8, after_8, before_8, picked_8): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let selected_10_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_10_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let selected_10_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_10 = { cantrip_mechanics::choose::select(g, selected_10_player, asker.source, Purpose::Exile, selected_10_candidates, cantrip_mechanics::select::Selection::of(selected_10_count, &[]))? };
let objects_9 = { let q: &dyn Query = g; selected_10.clone() }; let moved_9 = g.exile_all(&objects_9);
let paid_9 = moved_9.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_9, { let q: &dyn Query = g; (moved_9).clone() }, { let q: &dyn Query = g; (objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_10).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_8 as i64) != 0) } {
let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_11 = Vec::new();
let count_11_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_11_0 = g.card_id(&token_12::CARD);
for player in players_11 {
tokens_11.extend(std::iter::repeat((def_11_0,player)).take(count_11_0 as usize));
} let words_11 = g.words(asker.source);
let created_11 = g.create_tokens_worded(&tokens_11, false, None, words_11)?;
g.end_sentence();
hoisted_13.extend(created_11.clone());
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let value_14: i64 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Zombie)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64 };
g.end_sentence();
let amount_15 = { let q: &dyn Query = g; (value_14).clamp(0, 2147483647) as i32 }; let players_15 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_15 { g.gain_life(player, amount_15); }
g.end_sentence();
let amount_16 = { let q: &dyn Query = g; (value_14).clamp(0, 4294967295) as usize }; let player_16 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_16, asker.source, amount_16)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tymaret Calls the Dead", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II — Mill three cards. Then you may exile a creature or enchantment card from your graveyard. If you do, create a 2/2 black Zombie creature token.").targets(&[]),chapter(2, ability_0_1, "I, II — Mill three cards. Then you may exile a creature or enchantment card from your graveyard. If you do, create a 2/2 black Zombie creature token.").targets(&[]),chapter(3, ability_0_2, "III — You gain X life and scry X, where X is the number of Zombies you control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_5::CARD,&token_12::CARD];
