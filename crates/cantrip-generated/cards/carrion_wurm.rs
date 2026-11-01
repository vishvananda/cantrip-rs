// Carrion Wurm
// status: planned
// requires: continuous rule CombatDamage NoAssignment
// requires: player reference FirstWilling
// Whenever this creature attacks or blocks, any player may exile three cards from their graveyard. If a player does, this creature assigns no combat damage this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let willing_0: Option<PlayerId> = planned_value!("player reference FirstWilling", None::<PlayerId> => cantrip_mechanics::choose::first_willing(g, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, asker.source)?);
let mut paid_5 = false; for bound_1000006 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_7 = { let q: &dyn Query = g; Some(bound_1000006).expect("a reference that always names a player") }; let (paid_7, after_7, before_7, picked_7): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_7, asker.source, Purpose::Optional)? {
let selected_9_player = { let q: &dyn Query = g; Some(bound_1000006).expect("a reference that always names a player") }; let selected_9_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && willing_0 == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let selected_9_count = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let selected_9 = if selected_9_candidates.len() as u64 >= u64::from(selected_9_count) { cantrip_mechanics::choose::select(g, selected_9_player, asker.source, Purpose::Exile, selected_9_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_9_count), &[]))? } else { Vec::new() };
let objects_8 = { let q: &dyn Query = g; selected_9.clone() }; let moved_8 = g.exile_all(&objects_8);
let paid_8 = moved_8.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_8, { let q: &dyn Query = g; (moved_8).clone() }, { let q: &dyn Query = g; (objects_8.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_9).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
if paid_7 { paid_5 = true; break; } }

g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
let objects_10 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let duration_10 = Duration::EndOfTurn;
let kind_10_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule CombatDamage NoAssignment", EffectKind::CantBlock => EffectKind::AssignsNoCombatDamage) };
if let Some(source) = source_10 { cantrip_api::instruction(g, |g| { for &object in &objects_10 { g.add_effect(source, object, kind_10_0_0, duration_10);  } }); }
g.end_sentence();
hoisted_11.extend(objects_10.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_23: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let willing_0: Option<PlayerId> = planned_value!("player reference FirstWilling", None::<PlayerId> => cantrip_mechanics::choose::first_willing(g, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, asker.source)?);
let mut paid_17 = false; for bound_1000018 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_19 = { let q: &dyn Query = g; Some(bound_1000018).expect("a reference that always names a player") }; let (paid_19, after_19, before_19, picked_19): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_19, asker.source, Purpose::Optional)? {
let selected_21_player = { let q: &dyn Query = g; Some(bound_1000018).expect("a reference that always names a player") }; let selected_21_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && willing_0 == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let selected_21_count = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let selected_21 = if selected_21_candidates.len() as u64 >= u64::from(selected_21_count) { cantrip_mechanics::choose::select(g, selected_21_player, asker.source, Purpose::Exile, selected_21_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_21_count), &[]))? } else { Vec::new() };
let objects_20 = { let q: &dyn Query = g; selected_21.clone() }; let moved_20 = g.exile_all(&objects_20);
let paid_20 = moved_20.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_20, { let q: &dyn Query = g; (moved_20).clone() }, { let q: &dyn Query = g; (objects_20.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_21).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
if paid_19 { paid_17 = true; break; } }

g.end_sentence();
if { let q: &dyn Query = g; ((paid_17 as i64) != 0) } {
let objects_22 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_22 = { let q: &dyn Query = g; Some(asker.source) }; let duration_22 = Duration::EndOfTurn;
let kind_22_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule CombatDamage NoAssignment", EffectKind::CantBlock => EffectKind::AssignsNoCombatDamage) };
if let Some(source) = source_22 { cantrip_api::instruction(g, |g| { for &object in &objects_22 { g.add_effect(source, object, kind_22_0_0, duration_22);  } }); }
g.end_sentence();
hoisted_23.extend(objects_22.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Carrion Wurm", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::Wurm]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks or blocks, any player may exile three cards from their graveyard. If a player does, this creature assigns no combat damage this turn.").targets(&[]),triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_1_event), ability_1, "Whenever this creature attacks or blocks, any player may exile three cards from their graveyard. If a player does, this creature assigns no combat damage this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
