// Apathy
// status: ready
// Enchant creature
// Enchanted creature doesn't untap during its controller's untap step.
// At the beginning of the upkeep of enchanted creature's controller, that player may discard a card at random. If the player does, untap that creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).map(|o: ObjectId| q.controller(o)) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let (carried_4_0, carried_4_1, carried_4_2,) = if let Some(present_4_0) = { let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).map(|o: ObjectId| q.controller(o)) } { if let Some(present_4_1) = { let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).map(|o: ObjectId| q.controller(o)) } { let payer_2 = { let q: &dyn Query = g; present_4_1 }; let (paid_2, after_2, before_2, picked_2): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if g.state().zone(Zone::hand(payer_2)).len() >= 1 && cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)? {
let owed_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 } as usize; let player_3 = { let q: &dyn Query = g; present_4_0 }; let paid_3 = g.state().zone(Zone::hand(player_3)).len() >= owed_3; let hand_3 = g.state().zone(Zone::hand(player_3)).to_vec(); if paid_3 { cantrip_mechanics::discard_at_random(g, player_3, owed_3); } let discarded_3: Vec<ObjectId> = hand_3.iter().filter_map(|&o| g.state().obj(o).became).collect();
(paid_3, { let q: &dyn Query = g; (discarded_3).clone() }, { let q: &dyn Query = g; (discarded_3.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
(after_2.clone(), paid_2.clone(), { let q: &dyn Query = g; before_2.clone() },) } else { Default::default() } } else { Default::default() };
g.end_sentence();
if { let q: &dyn Query = g; ((carried_4_1 as i64) != 0) } {
let objects_5 = { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_5: Vec<ObjectId> = objects_5.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_5 { g.untap(object); } });
g.end_sentence();
hoisted_6.extend(turned_5.clone());
hoisted_7.extend(objects_5.clone());
hoisted_8.extend({ let q: &dyn Query = g; turned_5.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Apathy", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of the upkeep of enchanted creature's controller, that player may discard a card at random. If the player does, untap that creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_1_0_rule], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
