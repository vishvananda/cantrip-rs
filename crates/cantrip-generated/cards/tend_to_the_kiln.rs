// Tend to the Kiln
// status: planned
// requires: counter kind Flame
// requires: engine: no perpetual effects
// requires: operation Conjure
// When this enchantment enters, all non-Elemental instant and sorcery cards you own perpetually become Elemental kindreds in addition to their other types.
// Whenever you cast an Elemental spell, put a flame counter on this enchantment. Then if it has three or more flame counters on it, remove them, conjure a card named Flamebraider onto the battlefield, and it gains haste and "At the beginning of the end step, sacrifice this creature."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0: Vec<ObjectId> = { let q: &dyn Query = g; ({ let mut all: Vec<ObjectId> = q.state().battlefield().to_vec(); all.extend_from_slice(q.state().zone(Zone::STACK)); all.extend_from_slice(q.state().zone(Zone::COMMAND)); for p in q.state().seats() { for zone in [Zone::library(p), Zone::hand(p), Zone::graveyard(p), Zone::exile(p)] { all.extend_from_slice(q.state().zone(zone)); } } all }).into_iter().filter(|&id| ((!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elemental))) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() };
planned!("engine-no-perpetual-effects" => for &object in &objects_0 { cantrip_mechanics::perpetual::apply(g, asker.source, object, PerpetualEffect::AddCreatureType(CreatureType::Elemental)); cantrip_mechanics::perpetual::apply(g, asker.source, object, PerpetualEffect::AddCardtype(CardType::Kindred)); });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Elemental)) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn granted_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn granted_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_6: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_6 { let controller = g.controller(object); if players_6.contains(&controller) && g.can_sacrifice(controller, object) { picks_6.push((controller, object)); } }
let sacrificed_6 = g.sacrifice_each(&picks_6);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, planned_value!("counter kind Flame", CounterKind::Charge => CounterKind::Flame), amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(planned_value!("counter kind Flame", CounterKind::Charge => CounterKind::Flame)))) >= (3i64)) } {
let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = i16::MAX; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, planned_value!("counter kind Flame", CounterKind::Charge => CounterKind::Flame), amount_2_0)); } });
g.end_sentence();
let count_3: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_3: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_3.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Flamebraider"), count_3, ConjureTo::Battlefield { tapped: false })?)); }
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; ((conjured_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::Indefinite;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
let kind_4_1_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4); g.add_effect(source, object, kind_4_1_0, duration_4);  } }); }
g.end_sentence();
hoisted_7.extend(conjured_3.clone());
hoisted_8.extend(objects_4.clone());
hoisted_9.extend(conjured_3.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tend to the Kiln", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this enchantment enters, all non-Elemental instant and sorcery cards you own perpetually become Elemental kindreds in addition to their other types.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast an Elemental spell, put a flame counter on this enchantment. Then if it has three or more flame counters on it, remove them, conjure a card named Flamebraider onto the battlefield, and it gains haste and \"At the beginning of the end step, sacrifice this creature.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_5_event), granted_5, "At the beginning of the end step, sacrifice this creature.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
