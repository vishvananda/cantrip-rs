// Jared Carthalion, True Heir
// status: planned
// requires: engine: no static prevention with an effect besides
// requires: player rule for a while can't become the monarch
// When Jared Carthalion enters, target opponent becomes the monarch. You can't become the monarch this turn.
// If damage would be dealt to Jared Carthalion while you're the monarch, prevent that damage and put that many +1/+1 counters on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if let Some(player) = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) } { g.become_monarch(player); }
g.end_sentence();
planned!("player-rule-for-a-while-can-t-become-the-monarch" => let players_1_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for &player in &players_1_0 { g.add_player_effect(asker.controller, source, EffectKind::CantBecomeMonarch(player), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_0_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (i64::from(data[0])).min(i16::MAX.into()) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::PlusOne, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_1_0_then(g: &mut dyn Game, asker: &Asker, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_1_0_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) && Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).monarch) { return true; } }
 false }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-static-prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If damage would be dealt to Jared Carthalion while you're the monarch, prevent that damage and put that many +1/+1 counters on it.", applies: ability_1_0_applies, amount: None, then: Some(ability_1_0_then) });

pub static CARD: CardDef = CardDef { name: "Jared Carthalion, True Heir", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Jared Carthalion enters, target opponent becomes the monarch. You can't become the monarch this turn.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
