// The War Games
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I — Each player creates three tapped 1/1 white Warrior creature tokens. The tokens are goaded for as long as this Saga remains on the battlefield.
// II, III — Put a +1/+1 counter on each Warrior creature.
// IV — You may exile a nontoken creature you control. When you do, exile all Warriors.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Warrior).name, token_words: token!(Warrior).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, true, None, words_0)?;
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (created_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::WhileSourceOnBattlefield;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Goaded };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Warrior)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::PlusOne, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Warrior)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::PlusOne, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn reflexive_8(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_9 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Warrior))).collect::<Vec<ObjectId>>() }; let moved_9 = g.exile_all(&objects_9);
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_5_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_5_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::Exile, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
if !selected_5.is_empty() { 
let objects_6 = { let q: &dyn Query = g; selected_5.clone() }; let moved_6 = g.exile_all(&objects_6);
if !moved_6.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_10.extend(moved_6.clone());
hoisted_11.extend(selected_5.clone());
hoisted_12.extend({ let q: &dyn Query = g; objects_6.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The War Games", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Each player creates three tapped 1/1 white Warrior creature tokens. The tokens are goaded for as long as this Saga remains on the battlefield.").targets(&[]),chapter(2, ability_0_1, "II, III — Put a +1/+1 counter on each Warrior creature.").targets(&[]),chapter(3, ability_0_2, "II, III — Put a +1/+1 counter on each Warrior creature.").targets(&[]),chapter(4, ability_0_3, "IV — You may exile a nontoken creature you control. When you do, exile all Warriors.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_8, "When you do, exile all Warriors.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
