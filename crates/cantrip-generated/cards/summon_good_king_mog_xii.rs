// Summon: Good King Mog XII
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I — Create two 1/2 white Moogle creature tokens with lifelink.
// II, III — Whenever you cast a noncreature spell this turn, create a token that's a copy of a non-Saga token you control.
// IV — Put two +1/+1 counters on each other Moogle you control.
// Flying, lifelink
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Moogle).name, token_words: token!(Moogle).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Moogle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && !(q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn watch_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let among_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let chosen_4 = cantrip_mechanics::choose::select(g, player_4, asker.source, Purpose::CopyWhich, among_4, cantrip_mechanics::select::cards(cantrip_mechanics::select::exactly(1)))?.first().copied();
let values_4 = { let q: &dyn Query = g; chosen_4.map(|original: ObjectId| q.copiable_values(original)) }; let count_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let created_4 = match values_4 { Some(values) => g.create_token_copies_tapped(values, player_4, count_4, false)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_6_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && !(q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn watch_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let among_7 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga))) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let chosen_7 = cantrip_mechanics::choose::select(g, player_7, asker.source, Purpose::CopyWhich, among_7, cantrip_mechanics::select::cards(cantrip_mechanics::select::exactly(1)))?.first().copied();
let values_7 = { let q: &dyn Query = g; chosen_7.map(|original: ObjectId| q.copiable_values(original)) }; let count_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let created_7 = match values_7 { Some(values) => g.create_token_copies_tapped(values, player_7, count_7, false)?, None => Vec::new() };
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::ThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_8: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (2i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Moogle)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_8.push((object, CounterKind::PlusOne, amount)); } }
let had_8: Vec<_> = counters_8.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_8); let applied_8: Vec<ObjectId> = counters_8.iter().zip(had_8).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Summon: Good King Mog XII", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Saga,Subtype::Moogle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Create two 1/2 white Moogle creature tokens with lifelink.").targets(&[]),chapter(2, ability_0_1, "II, III — Whenever you cast a noncreature spell this turn, create a token that's a copy of a non-Saga token you control.").targets(&[]),chapter(3, ability_0_2, "II, III — Whenever you cast a noncreature spell this turn, create a token that's a copy of a non-Saga token you control.").targets(&[]),chapter(4, ability_0_3, "IV — Put two +1/+1 counters on each other Moogle you control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_3_event), watch_3, "Whenever you cast a noncreature spell this turn, create a token that's a copy of a non-Saga token you control.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_6_event), watch_6, "Whenever you cast a noncreature spell this turn, create a token that's a copy of a non-Saga token you control.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
