// Tempt with Bunnies
// status: ready
// Tempting Offer — Draw a card and create a 1/1 white Rabbit creature token. Then each opponent may draw a card and create a 1/1 white Rabbit creature token. For each opponent who does, you draw a card and you create a 1/1 white Rabbit creature token.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rabbit).name, token_words: token!(Rabbit).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rabbit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_8 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rabbit).name, token_words: token!(Rabbit).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rabbit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
pub mod token_12 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rabbit).name, token_words: token!(Rabbit).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rabbit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).is_some_and(|player: PlayerId| !cantrip_api::cant_draw(q, player, 1))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_3.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_5 = vec![Default::default(); players_3.len()];
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let payer_4 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let paid_4 = g.may_pay(payer_4, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
 each_5[index_3] = paid_4;
   } 
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_3.iter().position(|&next| next == p)).is_some_and(|at| (each_5[at] as i64) != 0)).collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
let players_7 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_3.iter().position(|&next| next == p)).is_some_and(|at| (each_5[at] as i64) != 0)).collect::<Vec<PlayerId>>() }; let mut tokens_7 = Vec::new();
let count_7_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_7_0 = g.card_id(&token_8::CARD);
for player in players_7 {
tokens_7.extend(std::iter::repeat((def_7_0,player)).take(count_7_0 as usize));
} let words_7 = g.words(asker.source);
let created_7 = g.create_tokens_worded(&tokens_7, false, None, words_7)?;
g.end_sentence();
let mut runs_9: u32 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_3.iter().position(|&next| next == p)).is_some_and(|at| (each_5[at] as i64) != 0)).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; while runs_9 > 0 { runs_9 -= 1; let mut again_9 = false;
let amount_10 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_10 { g.draw(player, amount_10); }
g.end_sentence();
let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_11 = Vec::new();
let count_11_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_11_0 = g.card_id(&token_12::CARD);
for player in players_11 {
tokens_11.extend(std::iter::repeat((def_11_0,player)).take(count_11_0 as usize));
} let words_11 = g.words(asker.source);
let created_11 = g.create_tokens_worded(&tokens_11, false, None, words_11)?;
g.end_sentence();
hoisted_13.extend(created_11.clone());
if !again_9 { break; } }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Tempt with Bunnies", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD,&token_8::CARD,&token_12::CARD];
