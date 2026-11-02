// Great Desert Hellion
// status: planned
// requires: engine: no Intensity
// requires: engine: no StartingIntensity
// requires: intensity
// Starting intensity 1
// Menace
// At the beginning of your upkeep, sacrifice this creature unless you discard a card. If you discard a card this way, this creature intensifies.
// When this creature leaves the battlefield, you may discard your hand. If you do, draw cards equal to this creature's intensity.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if g.state().zone(Zone::hand(payer_0)).len() >= 1 && cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_2)?); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
(paid_1, { let q: &dyn Query = g; (discarded_2).clone() }, { let q: &dyn Query = g; (discarded_2.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let objects_3 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
hoisted_4.extend(sacrificed_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
if { let q: &dyn Query = g; (((before_0.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
planned!("engine-no-intensity" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = 1; cantrip_mechanics::intensity::intensify(g, player, asker.source, amount); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_7 = cantrip_mechanics::effects::ask_may(g, payer_7, asker.source, Purpose::Optional)?; if paid_7 {
let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_8 = Vec::new(); for player in players_8 { cards_8.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_8 = g.discard_together(&cards_8);
g.end_sentence();
hoisted_9.extend(discarded_8.clone());
hoisted_10.extend({ let q: &dyn Query = g; discarded_8.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_7 as i64) != 0) } {
let amount_11 = { let q: &dyn Query = g; (Some(asker.source).map_or(0i64, |o: ObjectId| planned_value!("intensity", 0i64 => i64::from(q.state().obj(o).intensity)))).max(0) as u32 }; let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_11 { g.draw(player, amount_11); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-starting-intensity" => enters: cantrip_mechanics::intensity::starting::<1>);

pub static CARD: CardDef = CardDef { name: "Great Desert Hellion", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hellion]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of your upkeep, sacrifice this creature unless you discard a card. If you discard a card this way, this creature intensifies.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When this creature leaves the battlefield, you may discard your hand. If you do, draw cards equal to this creature's intensity.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
