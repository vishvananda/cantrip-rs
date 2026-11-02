// Nine Lives
// status: planned
// requires: engine: no static prevention with an effect besides
// Hexproof
// If a source would deal damage to you, prevent that damage and put an incarnation counter on this enchantment.
// When there are nine or more incarnation counters on this enchantment, exile it.
// When this enchantment leaves the battlefield, you lose the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Incarnation, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_1_then(g: &mut dyn Game, asker: &Asker, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_1_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return true; } }
 false }
fn ability_2_state(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Incarnation))) >= (9i64)) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-static-prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If a source would deal damage to you, prevent that damage and put an incarnation counter on this enchantment.", applies: ability_1_applies, amount: None, then: Some(ability_1_then) });

pub static CARD: CardDef = CardDef { name: "Nine Lives", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Hexproof]), protection: ColorSet(0), spell: None, abilities: &[state_triggered(ability_2_state, ability_2, "When there are nine or more incarnation counters on this enchantment, exile it."),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When this enchantment leaves the battlefield, you lose the game.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
