// Oversimplify
// status: ready
// Exile all creatures. Each player creates a 0/0 green and blue Fractal creature token and puts a number of +1/+1 counters on it equal to the total power of creatures they controlled that were exiled this way.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Fractal).name, token_words: token!(Fractal).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fractal]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(18)), power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let players_1 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<(PlayerId, ObjectId, CounterKind, i16)> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let mut counters_6: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (((objects_0.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter_map(|o: ObjectId| q.chars(o).power).map(i64::from).sum::<i64>()).clamp(0, 32767) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (created_1.clone()).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_6.push((object, CounterKind::PlusOne, amount)); } }
gathered_4.extend(counters_6.iter().filter(|c| g.state().obj(c.0).controller == bound_0).map(|c| (bound_0, c.0, c.1, c.2)));
    } 
let had_4: Vec<_> = gathered_4.iter().map(|c| g.state().obj(c.1).counters(c.2)).collect(); g.add_counters_together_by(&gathered_4); let applied_4: Vec<ObjectId> = gathered_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.1).counters(c.2) > *had).map(|(c, _)| c.1).collect();
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Oversimplify", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
