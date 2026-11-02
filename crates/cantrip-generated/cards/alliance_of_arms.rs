// Alliance of Arms
// status: ready
// Join forces — Starting with you, each player may pay any amount of mana. Each player creates X 1/1 white Soldier creature tokens, where X is the total amount of mana paid this way.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_5 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Soldier).name, token_words: token!(Soldier).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
let mut each_2 = vec![Default::default(); players_0.len()];
let mut done_total_2: i64 = 0;
let mut each_3 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { i64::from(g.may_pay_x(player, asker.source, asker.source, cost("{X}"), &PAY_QUESTION)?.unwrap_or(0)) }; }
 each_2[index_0] = done_1;
done_total_2 += done_1;
each_3[index_0] = done_1;
   } 
g.end_sentence();
let players_4 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut tokens_4 = Vec::new();
let count_4_0 = { let q: &dyn Query = g; ((done_total_2 as i64)) as u32 }; let def_4_0 = g.card_id(&token_5::CARD);
for player in players_4 {
tokens_4.extend(std::iter::repeat((def_4_0,player)).take(count_4_0 as usize));
} let words_4 = g.words(asker.source);
let created_4 = g.create_tokens_worded(&tokens_4, false, None, words_4)?;
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Alliance of Arms", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_5::CARD];
