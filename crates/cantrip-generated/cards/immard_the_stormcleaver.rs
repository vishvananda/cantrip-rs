// Immard, the Stormcleaver
// status: ready
// Whenever Immard, the Stormcleaver enters or attacks, put a charge counter on it or remove one from it. When you remove a counter this way, choose one —
// • Immard deals 4 damage to any target.
// • Immard gains lifelink and indestructible until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
static REFLEXIVE_5_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
fn reflexive_5_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let amount_6 = { let q: &dyn Query = g; (4i64).max(0) as u16 }; let recipients_6 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_6 { let batch: Vec<(ObjectId, Target, u16)> = recipients_6.iter().map(|t| (source, *t, amount_6)).collect(); g.damage_together(&batch); }
g.end_sentence();
 Ok(()) }
fn reflexive_5_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_7 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let duration_7 = Duration::EndOfTurn;
let kind_7_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])) };
let kind_7_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_7 { cantrip_api::instruction(g, |g| { for &object in &objects_7 { g.add_effect(source, object, kind_7_0_0, duration_7); g.add_effect(source, object, kind_7_1_0, duration_7);  } }); }
g.end_sentence();
 Ok(()) }
static REFLEXIVE_5_MODES: ModalDef = modes(&[mode(reflexive_5_mode_0, "Immard deals 4 damage to any target.").targets(REFLEXIVE_5_MODE_0_TARGETS),mode(reflexive_5_mode_1, "Immard gains lifelink and indestructible until end of turn.").targets(&[])]).choose(1, 1);
fn reflexive_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let offered_0: Vec<u8> = [(0u8, true), (1u8, { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().any(|o: ObjectId| q.state().obj(o).counters(CounterKind::Charge) > 0) })].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_0 = item.ability.card; let picked_0 = if offered_0.len() > 1 { offered_0[cantrip_mechanics::choose::one_of(g, decider_0, asker.source, offered_0.iter().map(|&option| Label::Card(card_0, option)).collect())? as usize] } else { offered_0.first().copied().unwrap_or(u8::MAX) }; match picked_0 {
0 => {
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::Charge, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_2.extend(applied_1.clone());
}
1 => {
let mut removed_3: i64 = 0;
let objects_3_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_3_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_3_0 { removed_3 += i64::from(g.remove_counters(object, CounterKind::Charge, amount_3_0)); } });
if removed_3 > 0 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
}
_ => {}
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Immard, the Stormcleaver", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever Immard, the Stormcleaver enters or attacks, put a charge counter on it or remove one from it. When you remove a counter this way, choose one — Immard deals 4 damage to any target; or Immard gains lifelink and indestructible until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_5, "When you remove a counter this way, choose one —").targets(&[]).modal(&REFLEXIVE_5_MODES)], enters: None, modal: None, enchant: None, labels: &["Put a charge counter on it", "Remove one from it"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
