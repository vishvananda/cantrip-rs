// Sivriss, Nightmare Speaker
// status: planned
// requires: per-player Mill
// requires: per-player instruction with several steps
// {T}, Sacrifice another creature or an artifact: For each opponent, you mill a card, then return that card from your graveyard to your hand unless that player pays 3 life. (To mill a card, put the top card of your library into your graveyard.)
// Choose a Background (You can have a Background as a second commander.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
planned!("per-player-mill" => g.begin_batch(););
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_0, _) in players_0.iter().copied().enumerate() { let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mills_1: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_1)).collect(); let milled_1 = g.mill_together(&mills_1);
 each_2[index_0] = milled_1.clone(); all_2.extend(each_2[index_0].iter().copied());
each_3[index_0] = { let q: &dyn Query = g; milled_1.clone() }; all_3.extend(each_3[index_0].iter().copied());
   } 
planned!("per-player-mill" => g.end_batch(););
g.end_sentence();
let mut each_5 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let payer_4 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let paid_4 = g.may_pay(payer_4, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(3)), &PAY_QUESTION)?;
 each_5[index_0] = paid_4;
   } 
g.end_sentence();
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
 for (index_0, _) in players_0.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_5[index_0] as i64) != 0)) } { guard_0_0[index_0] = true;
let objects_6 = { let q: &dyn Query = g; (each_2[index_0].clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_6 = g.return_to_hand(&objects_6);
 each_7[index_0] = moved_6.clone(); all_7.extend(each_7[index_0].iter().copied());
each_8[index_0] = { let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_8.extend(each_8[index_0].iter().copied());
  }
 } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_9(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) || q.chars(id).is(CardType::Artifact)) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Sivriss, Nightmare Speaker", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Snake,Subtype::Cleric,Subtype::Warlock]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(sacrifice(1, ability_0_9)), ability_0, "{T}, Sacrifice another creature or an artifact: For each opponent, you mill a card, then return that card from your graveyard to your hand unless that player pays 3 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
