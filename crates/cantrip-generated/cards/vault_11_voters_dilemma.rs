// Vault 11: Voter's Dilemma
// status: planned
// requires: condition Outcome
// requires: object state MostVotes
// requires: operation RevealChoices
// requires: operation Vote
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
// I — For each opponent, you create a 1/1 white Human Soldier creature token.
// II, III — Each player secretly votes for up to one creature, then those votes are revealed. If no creature got votes, each player draws a card. Otherwise, destroy each creature with the most votes or tied for most votes.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Human Soldier).name, token_words: token!(Human Soldier).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { let voters_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_3, cantrip_mechanics::vote::Ballot::UpToOne({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }), true)?);
    } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Object));
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition Outcome", false => q.outcome(asker, Outcome::NoVotes)) } {
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
} else {
let objects_6 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| planned_value!("object state MostVotes", false => votes.most(id))).collect::<Vec<ObjectId>>() }; let destroyed_6 = g.destroy_all(&objects_6);
g.end_sentence();
hoisted_7.extend(destroyed_6.clone());
hoisted_8.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let voters_10: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_10, cantrip_mechanics::vote::Ballot::UpToOne({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }), true)?);
    } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Object));
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition Outcome", false => q.outcome(asker, Outcome::NoVotes)) } {
let amount_12 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_12 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_12 { g.draw(player, amount_12); }
g.end_sentence();
} else {
let objects_13 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| planned_value!("object state MostVotes", false => votes.most(id))).collect::<Vec<ObjectId>>() }; let destroyed_13 = g.destroy_all(&objects_13);
g.end_sentence();
hoisted_14.extend(destroyed_13.clone());
hoisted_15.extend({ let q: &dyn Query = g; objects_13.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vault 11: Voter's Dilemma", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — For each opponent, you create a 1/1 white Human Soldier creature token.").targets(&[]),chapter(2, ability_0_1, "II, III — Each player secretly votes for up to one creature, then those votes are revealed. If no creature got votes, each player draws a card. Otherwise, destroy each creature with the most votes or tied for most votes.").targets(&[]),chapter(3, ability_0_2, "II, III — Each player secretly votes for up to one creature, then those votes are revealed. If no creature got votes, each player draws a card. Otherwise, destroy each creature with the most votes or tied for most votes.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
