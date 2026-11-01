// Regenerations Restored
// status: planned
// requires: engine: no VanishingEnters
// requires: engine: no event for a counter being removed
// Vanishing 12 (This enchantment enters with twelve time counters on it. At the beginning of your upkeep, remove a time counter from it. When the last is removed, sacrifice it.)
// Whenever one or more time counters are removed from this enchantment, scry 1 and you gain 1 life. Then if this enchantment has no time counters on it, exile it. When you do, take an extra turn after this one.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; g.add_extra_turns(player, times); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::scry(g, player_0, asker.source, amount_0)?;
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Time))) == (0i64)) } {
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
if !moved_2.clone().is_empty() && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
hoisted_6.extend(moved_2.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-vanishing-enters" => enters: cantrip_mechanics::vanishing::enters::<12>);
planned_item!("engine-no-event-for-a-counter-being-removed" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersRemoved { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if Some(asker.source) == Some(id) && *kind == CounterKind::Time { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Regenerations Restored", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no event for a counter being removed", EventMask(0) => EventMask(EventMask::COUNTERS_REMOVED.0)), ability_1_event), ability_1, "Whenever one or more time counters are removed from this enchantment, scry 1 and you gain 1 life. Then if this enchantment has no time counters on it, exile it. When you do, take an extra turn after this one.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "Whenever one or more time counters are removed from this enchantment, scry 1 and you gain 1 life. Then if this enchantment has no time counters on it, exile it. When you do, take an extra turn after this one.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
