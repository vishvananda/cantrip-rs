// Rayne, Academy Chancellor
// status: ready
// Whenever you or a permanent you control becomes the target of a spell or ability an opponent controls, you may draw a card. You may draw an additional card if Rayne is enchanted.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BecameTarget { target: Target::Player(target), by, controller, .. } = event { let by = *by; let controller = *controller; let subject = Target::Player(*target); let data = [by.0, u32::from(controller.0) | (q.state().stack.len().saturating_sub(1) as u32) << 8]; if Some(asker.controller) == Some(*target) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&controller) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if !cant_draw(g, payer_0, 1) && cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_2: i64 = 0; let mut drawn_2: Vec<ObjectId> = Vec::new(); for player in players_2 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_2); done_2 += g.history().drawn(player) as i64 - before; drawn_2.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
let paid_1 = done_2 > 0;
(paid_1, { let q: &dyn Query = g; (drawn_2).clone() }, Vec::new(), Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.state().battlefield().iter().any(|&a| q.state().obj(a).attached_to == Some(candidate) && q.chars(a).has_subtype(Subtype::Aura))) } {
let decider_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !cant_draw(g, decider_3, 1) && cantrip_mechanics::effects::ask_may(g, decider_3, asker.source, Purpose::Optional)? {
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BecameTarget { target: Target::Object(target), by, controller, .. } = event { let id = *target; let by = *by; let controller = *controller; let subject = Target::Object(id); let data = [by.0, u32::from(controller.0) | (q.state().stack.len().saturating_sub(1) as u32) << 8]; if Some(asker.controller) == Some(q.controller(id)) && q.is_on_battlefield(id) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&controller) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_5, after_5, before_5, picked_5): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if !cant_draw(g, payer_5, 1) && cantrip_mechanics::effects::ask_may(g, payer_5, asker.source, Purpose::Optional)? {
let amount_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_7: i64 = 0; let mut drawn_7: Vec<ObjectId> = Vec::new(); for player in players_7 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_7); done_7 += g.history().drawn(player) as i64 - before; drawn_7.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
let paid_6 = done_7 > 0;
(paid_6, { let q: &dyn Query = g; (drawn_7).clone() }, Vec::new(), Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.state().battlefield().iter().any(|&a| q.state().obj(a).attached_to == Some(candidate) && q.chars(a).has_subtype(Subtype::Aura))) } {
let decider_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if !cant_draw(g, decider_8, 1) && cantrip_mechanics::effects::ask_may(g, decider_8, asker.source, Purpose::Optional)? {
let amount_9 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.draw(player, amount_9); }
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rayne, Academy Chancellor", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::BECAME_TARGET.0), ability_0_event), ability_0, "Whenever you or a permanent you control becomes the target of a spell or ability an opponent controls, you may draw a card. You may draw an additional card if Rayne is enchanted.").targets(&[]),triggered(when_asked(EventMask(EventMask::BECAME_TARGET.0), ability_1_event), ability_1, "Whenever you or a permanent you control becomes the target of a spell or ability an opponent controls, you may draw a card. You may draw an additional card if Rayne is enchanted.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
