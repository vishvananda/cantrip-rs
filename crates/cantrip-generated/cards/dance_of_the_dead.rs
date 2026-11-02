// Dance of the Dead
// status: planned
// requires: enchant graveyard card
// requires: reflexive ability about two objects
// Enchant creature card in a graveyard
// When this Aura enters, if it's on the battlefield, it loses "enchant creature card in a graveyard" and gains "enchant creature put onto the battlefield with this Aura." Put enchanted creature card onto the battlefield tapped under your control and attach this Aura to it. When this Aura leaves the battlefield, that creature's controller sacrifices it.
// Enchanted creature gets +1/+1 and doesn't untap during its controller's untap step.
// At the beginning of the upkeep of enchanted creature's controller, that player may pay {1}{B}. If the player does, untap that creature.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (q.state().obj(candidate).became.is_none() && q.state().obj(candidate).zone.kind() == ZoneKind::Battlefield)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn delayed_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("reflexive-ability-about-two-objects", Sacrifice, "choose" = "()", "objects" = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }, "players" = "{\"args\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"ControllerOf\"},\"kind\":\"One\"}");
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let duration_0 = Duration::Indefinite;
let kind_0_0_0 = { let q: &dyn Query = g; EffectKind::LoseEnchant({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }) };
let kind_0_1_0 = { let q: &dyn Query = g; EffectKind::Enchant({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }) };
if let Some(source) = source_0 { cantrip_api::instruction(g, |g| { for &object in &objects_0 { g.add_effect(source, object, kind_0_0_0, duration_0); g.add_effect(source, object, kind_0_1_0, duration_0);  } }); }
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_1 = ({ let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_1 = { let rows_1: Vec<(ObjectId, PlayerId, bool)> = objects_1.iter().map(|&o| (o, player_1, true)).collect(); let came_1 = g.put_all_onto_battlefield_as(&rows_1)?; came_1.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
// Attached as it entered with this Aura.
g.end_sentence();
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilEvent, source, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied() }.map_or(Target::None, Target::Object)); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_3_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.state().obj(asker.source).attached_to == Some(id) }
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).map(|o: ObjectId| q.controller(o)) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let (carried_8_0,) = if let Some(present_8_0) = { let q: &dyn Query = g; Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).map(|o: ObjectId| q.controller(o)) } { let payer_7 = { let q: &dyn Query = g; present_8_0 }; let paid_7 = g.may_pay(payer_7, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
(paid_7.clone(),) } else { Default::default() };
g.end_sentence();
if { let q: &dyn Query = g; ((carried_8_0 as i64) != 0) } {
let objects_9 = { let q: &dyn Query = g; (Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_9: Vec<ObjectId> = objects_9.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_9 { g.untap(object); } });
g.end_sentence();
hoisted_10.extend(turned_9.clone());
hoisted_11.extend(objects_9.clone());
hoisted_12.extend({ let q: &dyn Query = g; turned_9.clone() });
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("enchant-graveyard-card" => enchant: Some(cantrip_mechanics::targets::graveyard_card(|q: &dyn Query, asker: &Asker, owner: PlayerId| true, |q: &dyn Query, asker: &Asker, id: ObjectId| -> bool { q.chars(id).is(CardType::Creature) })));

pub static CARD: CardDef = CardDef { name: "Dance of the Dead", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this Aura enters, if it's on the battlefield, it loses \"enchant creature card in a graveyard\" and gains \"enchant creature put onto the battlefield with this Aura.\" Put enchanted creature card onto the battlefield tapped under your control and attach this Aura to it. When this Aura leaves the battlefield, that creature's controller sacrifices it.").targets(&[]).condition(ability_1_if),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_4_event), ability_4, "At the beginning of the upkeep of enchanted creature's controller, that player may pay {1}{B}. If the player does, untap that creature.").targets(&[])], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Enchanted creature gets +1/+1 and doesn't untap during its controller's untap step.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_3_0_rule], delayed: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), delayed_4_event), delayed_4, "When this Aura leaves the battlefield, that creature's controller sacrifices it.").targets(&[])], enters: None, modal: None, enchant: None, becomes_aura_enchant: Some(cantrip_mechanics::targets::CREATURE_PUT_ONTO_THE_BATTLEFIELD_WITH_THIS), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
