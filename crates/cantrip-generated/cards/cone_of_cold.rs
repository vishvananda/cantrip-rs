// Cone of Cold
// status: planned
// requires: continuous rule UntapStep
// requires: duration DuringControllersNextUntapStep
// requires: engine: no resolving rule on an open set: UntapStep
// Roll a d20.
// 1—9 | Tap all creatures your opponents control.
// 10—19 | Tap all creatures your opponents control. Those creatures don't untap during their controllers' next untap steps.
// 20 | Tap all creatures your opponents control. Those creatures don't untap during their controllers' next untap steps. Until your next turn, creatures your opponents control enter tapped.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn made_13_applies(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&z.controller)) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn made_13_apply(q: &dyn Query, asker: &Asker, affected: ObjectId, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.tapped = true; } } }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_0) >= (1i64) && (value_0) <= (9i64)) } {
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_1);
g.end_sentence();
hoisted_2.extend(turned_1.clone());
hoisted_3.extend(objects_1.clone());
hoisted_4.extend({ let q: &dyn Query = g; turned_1.clone() });
}
if { let q: &dyn Query = g; ((value_0) >= (10i64) && (value_0) <= (19i64)) } {
let objects_5 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_5: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_5);
g.end_sentence();
planned!("engine-no-resolving-rule-on-an-open-set-untap-step" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))), planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap), planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep)); });
g.end_sentence();
hoisted_7.extend(turned_5.clone());
hoisted_8.extend(objects_5.clone());
hoisted_9.extend({ let q: &dyn Query = g; turned_5.clone() });
}
if { let q: &dyn Query = g; (value_0) == (20i64) } {
let objects_10 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_10: Vec<ObjectId> = objects_10.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_10);
g.end_sentence();
planned!("engine-no-resolving-rule-on-an-open-set-untap-step" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { g.add_open_effect(source, move |q: &dyn Query, asker: &Asker, id: ObjectId| (q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))), planned_value!("continuous rule UntapStep", EffectKind::CantBlock => EffectKind::DoesntUntap), planned_value!("duration DuringControllersNextUntapStep", Duration::Indefinite => Duration::DuringItsControllersNextUntapStep)); });
g.end_sentence();
{  let to = Some(Target::None); let about = to.and_then(|to: Target| to.object()).or(Some(item.object)); if let (Some(to), Some(about)) = (to, about) { let _ = (to, about); g.add_player_effect(item.controller, item.object, EffectKind::ReplacementRule { card: item.ability.card, index: 0 }, Duration::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") })); } }
g.end_sentence();
hoisted_14.extend(turned_10.clone());
hoisted_15.extend(objects_10.clone());
hoisted_16.extend({ let q: &dyn Query = g; turned_10.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Cone of Cold", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_replacements: &[granted_replacement(ReplacementKind::ZoneChange, made_13_applies, made_13_apply, "Roll a d20.").text("Roll a d20.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
