// Shared Trauma
// status: planned
// requires: an amount read by each player as their own
// Join forces — Starting with you, each player may pay any amount of mana. Each player mills X cards, where X is the total amount of mana paid this way.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
let mut each_2 = vec![Default::default(); players_0.len()];
let mut each_3 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { i64::from(g.may_pay_x(player, asker.source, asker.source, cost("{X}"), &PAY_QUESTION)?.unwrap_or(0)) }; }
 each_2[index_0] = done_1;
each_3[index_0] = done_1;
   } 
g.end_sentence();
planned!("an-amount-read-by-each-player-as-their-own", Mill, "amount" = "{\"args\":0,\"kind\":\"Performed\"}", "players" = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() });
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Shared Trauma", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
