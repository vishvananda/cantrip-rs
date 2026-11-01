// Vivien, Monsters' Advocate
// status: planned
// requires: instruction ChooseOne
// You may look at the top card of your library any time.
// You may cast creature spells from the top of your library.
// [+1]: Create a 3/3 green Beast creature token. Put your choice of a reach counter, a vigilance counter, or a trample counter on it.
// [−2]: When you next cast a creature spell this turn, search your library for a creature card with lesser mana value, put it onto the battlefield, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { q.chars(spell).is(CardType::Creature) }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Beast).name, token_words: token!(Beast).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(16)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_2, asker.source, vec![Label::Card(card_2, 0), Label::Card(card_2, 1), Label::Card(card_2, 2)])?) {
0 => {
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (created_0.clone()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::Reach, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_4.extend(applied_3.clone());
}
1 => {
let mut counters_5: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (created_0.clone()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_5.push((object, CounterKind::Vigilance, amount)); } }
let had_5: Vec<_> = counters_5.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_5); let applied_5: Vec<ObjectId> = counters_5.iter().zip(had_5).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_6.extend(applied_5.clone());
}
_ => {
let mut counters_7: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (created_0.clone()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_7.push((object, CounterKind::Trample, amount)); } }
let had_7: Vec<_> = counters_7.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_7); let applied_7: Vec<ObjectId> = counters_7.iter().zip(had_7).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_8.extend(applied_7.clone());
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_10_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).is(CardType::Creature) { return Some(Fired { subject, data }); } }
 None }
fn watch_10(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let searcher_11 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let owner_11 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let rule_count_11 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let shown_11 = cantrip_mechanics::library::searched_cards(g, owner_11); let can_11: Vec<ObjectId> = { let q: &dyn Query = g; shown_11.iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) < (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))))).collect() }; let found_11 = cantrip_mechanics::library::search(g, searcher_11, owner_11, asker.source, SearchDestination::Battlefield, cantrip_mechanics::select::Selection::listed(rule_count_11, &shown_11, can_11))?;
let player_12 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_12 = ({ let q: &dyn Query = g; found_11.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_12 = { let rows_12: Vec<(ObjectId, PlayerId, bool)> = objects_12.iter().map(|&o| (o, player_12, false)).collect(); let came_12 = g.put_all_onto_battlefield_as(&rows_12)?; came_12.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vivien, Monsters' Advocate", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Vivien]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(3), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_2, "+1: Create a 3/3 green Beast creature token. Put your choice of a reach counter, a vigilance counter, or a trample counter on it.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_3, "−2: When you next cast a creature spell this turn, search your library for a creature card with lesser mana value, put it onto the battlefield, then shuffle.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_10_event), watch_10, "−2: When you next cast a creature spell this turn, search your library for a creature card with lesser mana value, put it onto the battlefield, then shuffle.").targets(&[])], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop,PlayRule::LibraryTopSpells(ability_1_0_rule, None)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
