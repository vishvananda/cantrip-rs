// Ilharg, the Raze-Boar
// status: ready
// Trample
// Whenever Ilharg attacks, you may put a creature card from your hand onto the battlefield tapped and attacking. Return that creature to your hand at the beginning of the next end step.
// When Ilharg dies or is put into exile from the battlefield, you may put it into its owner's library third from the top.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn delayed_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; (about(q, item, 0)).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let put_0: Vec<ObjectId> = cantrip_mechanics::hand::may_put_onto_battlefield_attacking(g, payer_0, asker.source, |g, id| { let q: &dyn Query = g; q.chars(id).is(CardType::Creature) })?.into_iter().collect(); let paid_0 = !put_0.is_empty();
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; put_0.clone() }); about.push(ObjectId::NONE);  if about.iter().any(|&o| o != ObjectId::NONE) { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if ({ let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied() }).is_some() && cantrip_mechanics::effects::ask_may(g, decider_4, asker.source, Purpose::Optional)? {
let objects_5 = { let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_5 = g.put_into_library(&objects_5, LibraryPlace::FromTop(({ let q: &dyn Query = g; (3i64).max(0) as usize }).saturating_sub(1)));
g.end_sentence();
hoisted_6.extend(moved_5.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_5.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Exile) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_8, asker.source, Purpose::Optional)? {
let objects_9 = { let q: &dyn Query = g; (((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_9 = g.put_into_library(&objects_9, LibraryPlace::FromTop(({ let q: &dyn Query = g; (3i64).max(0) as usize }).saturating_sub(1)));
g.end_sentence();
hoisted_10.extend(moved_9.clone());
hoisted_11.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }

pub static CARD: CardDef = CardDef { name: "Ilharg, the Raze-Boar", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Boar,Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Ilharg attacks, you may put a creature card from your hand onto the battlefield tapped and attacking. Return that creature to your hand at the beginning of the next end step.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Whenever Ilharg attacks, you may put a creature card from your hand onto the battlefield tapped and attacking. Return that creature to your hand at the beginning of the next end step.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When Ilharg dies or is put into exile from the battlefield, you may put it into its owner's library third from the top.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_2, "Return that creature to your hand at the beginning of the next end step.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
