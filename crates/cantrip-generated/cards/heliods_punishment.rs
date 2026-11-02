// Heliod's Punishment
// status: ready
// Enchant creature
// This Aura enters with four task counters on it.
// Enchanted creature can't attack or block. It loses all abilities and has "{T}: Remove a task counter from Heliod's Punishment. Then if it has no task counters on it, destroy Heliod's Punishment."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (4i64).max(0) as i16 }; entering.counters[CounterKind::Task as usize] = entering.counters[CounterKind::Task as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(attacker)) }
#[allow(unused_variables)] fn ability_2_1_rule(q: &dyn Query, source: ObjectId, blocker: ObjectId, attacker: ObjectId) -> bool { let asker = &Asker::new(source, q.controller(source)); !(q.state().obj(asker.source).attached_to == Some(blocker) && true) }
fn granted_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; granter(q, item).into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, CounterKind::Task, amount_2_0)); } });
g.end_sentence();
if { let q: &dyn Query = g; granter(q, item).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Task))) == (0i64)) } {
let objects_3 = { let q: &dyn Query = g; granter(q, item).into_iter().collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
hoisted_4.extend(destroyed_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
fn granter(q: &dyn Query, item: &StackItem) -> Option<ObjectId> { if item.acquired.by != AcquiredBy::Grant { return None; } let stamp = item.acquired.stamp; let card = item.ability.card; if let Some(e) = q.state().effects.iter().find(|e| e.timestamp == stamp && matches!(e.kind, EffectKind::AddAbility(_))) { return Some(e.source); } if let Some(id) = q.state().battlefield().iter().copied().find(|&id| q.state().obj(id).timestamp == stamp && q.chars(id).card == card) { return Some(id); } q.history().all().iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, from, .. } if from.is(ZoneKind::Battlefield) && q.state().obj(old).timestamp == stamp && q.state().obj(old).card == card => Some(old), _ => None }) }

pub static CARD: CardDef = CardDef { name: "Heliod's Punishment", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[loses_all_abilities(cantrip_mechanics::attach::attached_object, "Enchanted creature can't attack or block. It loses all abilities and has \"{T}: Remove a task counter from Heliod's Punishment. Then if it has no task counters on it, destroy Heliod's Punishment.\""),static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_3_apply_1, "Enchanted creature can't attack or block. It loses all abilities and has \"{T}: Remove a task counter from Heliod's Punishment. Then if it has no task counters on it, destroy Heliod's Punishment.\"")], replacements: &[], preventions: &[], block_rules: &[ability_2_1_rule], block_requirements: &[], attack_rules: &[ability_2_0_rule], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), granted_1, "{T}: Remove a task counter from Heliod's Punishment. Then if it has no task counters on it, destroy Heliod's Punishment.").targets(&[])], enters: Some(ability_1_enters), modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
