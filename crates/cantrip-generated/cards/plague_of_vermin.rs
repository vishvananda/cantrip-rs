// Plague of Vermin
// status: ready
// Starting with you, each player may pay any amount of life. Repeat this process until no one pays life. Each player creates a 1/1 black Rat creature token for each 1 life they paid this way.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_12 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Rat).name, token_words: token!(Rat).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut shares_0: Vec<(PlayerId, i64)> = Vec::new();
loop { let mut again_1 = false;
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_2 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_2.iter().position(|&p| Some(p) == first_2) { players_2.rotate_left(at); }
let mut each_4 = vec![Default::default(); players_2.len()];
let mut done_total_4: i64 = 0;
let mut each_5 = vec![Default::default(); players_2.len()];
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let mut done_3: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { done_3 += { let most = g.state().player(player).life.clamp(0, 255) as u32; let paid = cantrip_mechanics::choose::number(g, player, asker.source, 0, most, "Pay")?; if paid > 0 && g.may_pay(player, asker.source, asker.source, life(paid as u8), &PAY_QUESTION)? { i64::from(paid) } else { 0 } }; }
 each_4[index_2] = done_3;
done_total_4 += done_3;
shares_0.push((bound_0, (done_3) as i64));
each_5[index_2] = done_3;
   } 
g.end_sentence();
if { let q: &dyn Query = g; !((done_total_4 == 0)) } {
again_1 = true;
g.end_sentence();
}
g.end_sentence();
if !again_1 { break; } }
g.end_sentence();
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_10: Vec<(CardId, PlayerId)> = Vec::new();
 for (index_9, bound_1048576) in players_9.iter().copied().enumerate() { let def_11 = g.card_id(&token_12::CARD); let count_11 = { let q: &dyn Query = g; ((shares_0.iter().filter(|share| share.0 == bound_1048576).map(|share| share.1).sum::<i64>() as i64)).max(0) as u32 }; let player_11 = { let q: &dyn Query = g; Some(bound_1048576).expect("a reference that always names a player") }; gathered_10.extend(std::iter::repeat((def_11, player_11)).take(count_11 as usize));
    } 
let words_10 = g.words(asker.source); let created_10 = g.create_tokens_worded(&gathered_10, false, None, words_10)?;
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Plague of Vermin", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_12::CARD];
