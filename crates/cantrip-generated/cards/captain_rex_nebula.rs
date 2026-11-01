// Captain Rex Nebula
// status: ready
// At the beginning of combat on your turn, choose target nonland permanent you control. Until end of turn, it becomes a Vehicle artifact with base power and toughness each equal to its mana value, and it gains crew 2 and "Crash Land — Whenever this Vehicle deals damage, roll a six-sided die. If the result is equal to this Vehicle's mana value, sacrifice this Vehicle, then it deals that much damage to any target."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).is(CardType::Land)) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
static GRANTED_2_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn granted_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && true { return Some(Fired { subject, data }); } }
 None }
fn granted_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_3 = i64::from(g.roll_die(player_3, 6).result);
g.end_sentence();
if { let q: &dyn Query = g; (value_3) == (Some(asker.source).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) } {
let objects_4 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_4 { let controller = g.controller(object); if players_4.contains(&controller) && g.can_sacrifice(controller, object) { picks_4.push((controller, object)); } }
let sacrificed_4 = g.sacrifice_each(&picks_4);
g.end_sentence();
let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let amount_5 = { let q: &dyn Query = g; (value_3).clamp(0, 65535) as u16 }; let recipients_5 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_5 { let batch: Vec<(ObjectId, Target, u16)> = recipients_5.iter().map(|t| (source, *t, amount_5)).collect(); g.damage_together(&batch); }
g.end_sentence();
hoisted_6.extend(sacrificed_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::EndOfTurn;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::LoseSubtypesOf(CardType::Artifact) };
let kind_0_1_1 = { let q: &dyn Query = g; EffectKind::AddSubtype(Subtype::Vehicle) };
let kind_0_2_0 = { let q: &dyn Query = g; EffectKind::SetTypes(TypeSet::of(&[CardType::Artifact])) };
let kind_0_3_0 = { let q: &dyn Query = g; EffectKind::SetPt((t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16, (t_0_raw.get(0).copied().and_then(|t: Target| t.object()).map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))).clamp(-32768, 32767) as i16) };
let kind_0_4_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
let kind_0_5_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1)) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_1, duration_0); g.add_effect(source, object, kind_0_2_0, duration_0); g.add_effect(source, object, kind_0_3_0, duration_0); g.add_effect(source, object, kind_0_4_0, duration_0); g.add_effect(source, object, kind_0_5_0, duration_0);  } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Captain Rex Nebula", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pilot,Subtype::Employee]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, choose target nonland permanent you control. Until end of turn, it becomes a Vehicle artifact with base power and toughness each equal to its mana value, and it gains crew 2 and \"Crash Land — Whenever this Vehicle deals damage, roll a six-sided die. If the result is equal to this Vehicle's mana value, sacrifice this Vehicle, then it deals that much damage to any target.\"").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::crew::crew(2, "Crew 2"),triggered(when_asked(EventMask(EventMask::DAMAGE.0), granted_2_event), granted_2, "Crash Land — Whenever this Vehicle deals damage, roll a six-sided die. If the result is equal to this Vehicle's mana value, sacrifice this Vehicle, then it deals that much damage to any target.").targets(GRANTED_2_TARGETS).sums_per_subject()], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
