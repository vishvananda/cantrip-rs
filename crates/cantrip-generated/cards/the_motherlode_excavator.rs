// The Motherlode, Excavator
// status: planned
// requires: event object this trigger does not bind
// When The Motherlode enters, choose target opponent. You get an amount of {E} (energy counters) equal to the number of nonbasic lands that player controls.
// Whenever The Motherlode attacks, you may pay {E}{E}{E}{E}. When you do, destroy target nonbasic land defending player controls, and creatures that player controls without flying can't block this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land) && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).min(i32::MAX.into()) as i32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_0, amount_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (((!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land)) && q.event_subject().object().and_then(|o: ObjectId| q.defending_player(o)) == Some(q.controller(id))))) }
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_4_target_0)];
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_5 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let destroyed_5 = g.destroy_all(&objects_5);
g.end_sentence();
planned!("event-object-this-trigger-does-not-bind", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"args\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"DefendingPlayerOf\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"},{\"args\":{\"args\":{\"kind\":\"Flying\"},\"kind\":\"HasKeyword\"},\"kind\":\"Not\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}", "changes" = "[{\"args\":{\"Prohibit\":{\"Block\":{\"kind\":\"Any\"}}},\"kind\":\"Rule\"}]", "duration" = "\"EndOfTurn\"", "evaluation" = "\"LiveObjects\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.state().player(decider_1).energy() >= 4 && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let owed_2: u8 = { let q: &dyn Query = g; (4i64).max(0) as u8 }; let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_2 = g.state().player(payer_2).energy() >= owed_2; if paid_2 { g.change_energy(payer_2, -i32::from(owed_2)); }
if paid_2 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; item.subject.object() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Motherlode, Excavator", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Robot]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When The Motherlode enters, choose target opponent. You get an amount of {E} equal to the number of nonbasic lands that player controls.").targets(ABILITY_0_TARGETS),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever The Motherlode attacks, you may pay {E}{E}{E}{E}. When you do, destroy target nonbasic land defending player controls, and creatures that player controls without flying can't block this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "When you do, destroy target nonbasic land defending player controls, and creatures that player controls without flying can't block this turn.").targets(REFLEXIVE_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
