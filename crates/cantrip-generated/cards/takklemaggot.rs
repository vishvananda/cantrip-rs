// Takklemaggot
// status: planned
// requires: move arrival state
// requires: object reference Scoped
// requires: power/toughness counter other than ±1/±1
// Enchant creature
// At the beginning of the upkeep of enchanted creature's controller, put a -0/-1 counter on that creature.
// When enchanted creature dies, that creature's controller chooses a creature that this card could enchant. If the player does, return this card to the battlefield under your control attached to that creature. If they don't, return this card to the battlefield under your control as a non-Aura enchantment. It loses "enchant creature" and gains "At the beginning of that player's upkeep, this enchantment deals 1 damage to that player."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).map(|o: ObjectId| q.controller(o)) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, planned_value!("power/toughness counter other than ±1/±1", CounterKind::PlusOne => CounterKind::Plus0Minus1), amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn granted_9_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && planned_value!("object reference Scoped", None::<ObjectId> => q.scoped_object(asker)).map(|o: ObjectId| q.controller(o)) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn granted_9(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let amount_10 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; let recipients_10 = { let q: &dyn Query = g; (item.subject.player().into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_10 { let batch: Vec<(ObjectId, Target, u16)> = recipients_10.iter().map(|t| (source, *t, amount_10)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let paid_1 = true; if paid_1 {
let selected_2_player = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().any(|host: ObjectId| could_enchant(q, id, asker.controller, host)))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::ChooseAsEnters, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
g.end_sentence();
hoisted_3.extend(selected_2.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_1 as i64) != 0) } {
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_4 { let controller = player_4; if let Some(host) = { let q: &dyn Query = g; (hoisted_3.clone()).get(0 as usize).copied() } { moved.extend(g.put_onto_battlefield_attached(o, controller, host)?); } } moved };
g.end_sentence();
hoisted_5.extend(moved_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
} else {
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_7 { let controller = player_7; moved.extend(planned_value!("move arrival state", None::<ObjectId> => cantrip_mechanics::enter::in_state(g, o, controller, item, "[NotAnAura]", &[], false)?)); } moved };
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; ((moved_7.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_8 = { let q: &dyn Query = g; Some(asker.source) }; let duration_8 = Duration::Indefinite;
let kind_8_0_0 = { let q: &dyn Query = g; EffectKind::LoseEnchant({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }) };
let kind_8_1_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
if let Some(source) = source_8 { cantrip_api::instruction(g, |g| { for &object in &objects_8 { g.add_effect(source, object, kind_8_0_0, duration_8); g.add_effect(source, object, kind_8_1_0, duration_8);  } }); }
g.end_sentence();
hoisted_11.extend(moved_7.clone());
hoisted_12.extend(objects_8.clone());
hoisted_13.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn could_enchant(q: &dyn Query, aura: ObjectId, controller: PlayerId, host: ObjectId) -> bool { let chars = q.chars(aura); if chars.is(CardType::Creature) { return false; } let asker = Asker::new(aura, controller); let printed = q.card(q.state().obj(aura).card).enchant; chars.abilities().filter_map(|a| match a.index { AbilityRef::ENCHANT => q.card(a.card).enchant, AbilityRef::ENCHANT_GIVEN => q.card(a.card).becomes_aura_enchant, _ => None }).chain(printed).all(|spec| (spec.filter)(q, &asker, Target::Object(host))) && !q.chars(host).protected_from_by(&chars, || q.mana_value(aura), controller) }

pub static CARD: CardDef = CardDef { name: "Takklemaggot", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of the upkeep of enchanted creature's controller, put a -0/-1 counter on that creature.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When enchanted creature dies, that creature's controller chooses a creature that this card could enchant. If the player does, return this card to the battlefield under your control attached to that creature. If they don't, return this card to the battlefield under your control as a non-Aura enchantment. It loses \"enchant creature\" and gains \"At the beginning of that player's upkeep, this enchantment deals 1 damage to that player.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_9_event), granted_9, "At the beginning of that player's upkeep, this enchantment deals 1 damage to that player.").targets(&[])], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
