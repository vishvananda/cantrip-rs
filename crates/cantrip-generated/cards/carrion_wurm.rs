// Carrion Wurm
// status: planned
// requires: continuous rule CombatDamage NoAssignment
// Whenever this creature attacks or blocks, any player may exile three cards from their graveyard. If a player does, this creature assigns no combat damage this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut paid_0 = false; let mut who_0: Option<PlayerId> = None; for bound_1000001 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_2 = { let q: &dyn Query = g; Some(bound_1000001).expect("a reference that always names a player") }; let (paid_2, after_2, before_2, picked_2): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)? {
let selected_4_player = { let q: &dyn Query = g; Some(bound_1000001).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_1000001) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let selected_4 = if selected_4_candidates.len() as u64 >= u64::from(selected_4_count) { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::Exile, selected_4_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_4_count), &[]))? } else { Vec::new() };
let objects_3 = { let q: &dyn Query = g; selected_4.clone() }; let moved_3 = g.exile_all(&objects_3);
let paid_3 = moved_3.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_3, { let q: &dyn Query = g; (moved_3).clone() }, { let q: &dyn Query = g; (objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_4).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
if paid_2 { paid_0 = true; who_0 = Some(bound_1000001); break; } }
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let objects_5 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::EndOfTurn;
let kind_5_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule CombatDamage NoAssignment", EffectKind::CantBlock => EffectKind::AssignsNoCombatDamage) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
g.end_sentence();
hoisted_6.extend(objects_5.clone());
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut paid_7 = false; let mut who_7: Option<PlayerId> = None; for bound_1000008 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_9 = { let q: &dyn Query = g; Some(bound_1000008).expect("a reference that always names a player") }; let (paid_9, after_9, before_9, picked_9): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_9, asker.source, Purpose::Optional)? {
let selected_11_player = { let q: &dyn Query = g; Some(bound_1000008).expect("a reference that always names a player") }; let selected_11_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_1000008) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }; let selected_11_count = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let selected_11 = if selected_11_candidates.len() as u64 >= u64::from(selected_11_count) { cantrip_mechanics::choose::select(g, selected_11_player, asker.source, Purpose::Exile, selected_11_candidates, cantrip_mechanics::select::Selection::of(cantrip_mechanics::select::Count::Exactly(selected_11_count), &[]))? } else { Vec::new() };
let objects_10 = { let q: &dyn Query = g; selected_11.clone() }; let moved_10 = g.exile_all(&objects_10);
let paid_10 = moved_10.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
(paid_10, { let q: &dyn Query = g; (moved_10).clone() }, { let q: &dyn Query = g; (objects_10.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, { let q: &dyn Query = g; (selected_11).clone() }) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
if paid_9 { paid_7 = true; who_7 = Some(bound_1000008); break; } }
g.end_sentence();
if { let q: &dyn Query = g; ((paid_7 as i64) != 0) } {
let objects_12 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_12 = { let q: &dyn Query = g; Some(asker.source) }; let duration_12 = Duration::EndOfTurn;
let kind_12_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule CombatDamage NoAssignment", EffectKind::CantBlock => EffectKind::AssignsNoCombatDamage) };
if let Some(source) = source_12 { cantrip_api::instruction(g, |g| { for &object in &objects_12 { g.add_effect(source, object, kind_12_0_0, duration_12);  } }); }
g.end_sentence();
hoisted_13.extend(objects_12.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Carrion Wurm", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::Wurm]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever this creature attacks or blocks, any player may exile three cards from their graveyard. If a player does, this creature assigns no combat damage this turn.").targets(&[]),triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_1_event), ability_1, "Whenever this creature attacks or blocks, any player may exile three cards from their graveyard. If a player does, this creature assigns no combat damage this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
