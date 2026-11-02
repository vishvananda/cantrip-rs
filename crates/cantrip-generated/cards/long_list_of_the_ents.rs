// Long List of the Ents
// status: planned
// requires: object filter Noted
// requires: operation ChangeSpell
// requires: operation ChooseValueWith
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after VI.)
// I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && planned_value!("object filter Noted", false => q.noted(asker, ChoiceKind::CreatureType).matches(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_0 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[ChoosableCreatureType::AnyCreatureType], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_6_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && planned_value!("object filter Noted", false => q.noted(asker, ChoiceKind::CreatureType).matches(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_6(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_4 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[ChoosableCreatureType::AnyCreatureType], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_10_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && planned_value!("object filter Noted", false => q.noted(asker, ChoiceKind::CreatureType).matches(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_10(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_8 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[ChoosableCreatureType::AnyCreatureType], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 2), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_14_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && planned_value!("object filter Noted", false => q.noted(asker, ChoiceKind::CreatureType).matches(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_14(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_12 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[ChoosableCreatureType::AnyCreatureType], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 3), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_18_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && planned_value!("object filter Noted", false => q.noted(asker, ChoiceKind::CreatureType).matches(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_18(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_16 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[ChoosableCreatureType::AnyCreatureType], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 4), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_22_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Creature) && planned_value!("object filter Noted", false => q.noted(asker, ChoiceKind::CreatureType).matches(id))) { return Some(Fired { subject, data }); } }
 None }
fn watch_22(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { let changes = { let q: &dyn Query = g; [SpellChange::EntersWithCounters(CounterKind::PlusOne, (1i64).max(0) as i16)] }; cantrip_mechanics::spell_changes::change(g, asker, spell, &changes); });
g.end_sentence();
 Ok(()) }
fn ability_0_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_20 = planned_value!("operation ChooseValueWith", Subtype::Plains => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::CreatureType, ChoiceTerms { options: &[ChoosableCreatureType::AnyCreatureType], except: &[], count: None, secret: false, unchosen: true, random: false, ordered: false, up_to: false })?);
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::NextThisTurn, source, item.controller, AbilityRef::delayed(item.ability.card, 5), Target::None); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Long List of the Ents", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),chapter(2, ability_0_1, "I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),chapter(3, ability_0_2, "I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),chapter(4, ability_0_3, "I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),chapter(5, ability_0_4, "I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),chapter(6, ability_0_5, "I, II, III, IV, V, VI — Note a creature type that hasn't been noted for this Saga. When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_2_event), watch_2, "When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_6_event), watch_6, "When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_10_event), watch_10, "When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_14_event), watch_14, "When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_18_event), watch_18, "When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_22_event), watch_22, "When you next cast a creature spell of that type this turn, that creature enters with an additional +1/+1 counter on it.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
