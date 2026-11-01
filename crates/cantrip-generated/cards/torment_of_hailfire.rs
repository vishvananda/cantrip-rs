// Torment of Hailfire
// status: ready
// Repeat the following process X times. Each opponent loses 3 life unless that player sacrifices a nonland permanent of their choice or discards a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn paid_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { !(q.chars(id).is(CardType::Land)) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut runs_0: u32 = { let q: &dyn Query = g; (i64::from(item.x)).max(0) as u32 }; while runs_0 > 0 { runs_0 -= 1; let mut again_0 = false;
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_1_0 = vec![false; players_1.len()];
let mut each_4 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let payer_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let paid_2 = g.may_pay(payer_2, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(1, paid_3)).and(discard_cards(1)), &PAY_QUESTION)?;
 each_4[index_1] = paid_2;
   } 
g.end_sentence();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_4[index_1] as i64) != 0)) } { guard_1_0[index_1] = true;
let amount_5 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.lose_life(player, amount_5); }
   }
 } 
g.end_sentence();
again_0 = true;
g.end_sentence();
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Torment of Hailfire", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
