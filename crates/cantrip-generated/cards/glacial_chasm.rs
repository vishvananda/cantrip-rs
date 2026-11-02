// Glacial Chasm
// status: ready
// Cumulative upkeep—Pay 2 life. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// When this land enters, sacrifice a land.
// Creatures you control can't attack.
// Prevent all damage that would be dealt to you.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_0 { let eligible: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_0.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_0.clone(), &[]))? }; for object in taken { picks_0.push((player, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, attacker: ObjectId, defender: Target) -> bool { let source = asker.source; let asker = &Asker::new(source, q.controller(source)); !((q.chars(attacker).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(attacker)))) }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Glacial Chasm", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Life(2), "Cumulative upkeep"),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this land enters, sacrifice a land.").targets(&[])], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all damage that would be dealt to you", applies: ability_3_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[ability_2_0_rule], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
