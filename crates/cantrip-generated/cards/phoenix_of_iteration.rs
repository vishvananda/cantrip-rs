// Phoenix of Iteration
// status: planned
// requires: engine: no perpetual effects
// Flying
// Opus — Whenever you cast an instant or sorcery spell, Phoenix of Iteration perpetually gets +1/+1. If five or more mana was spent to cast that spell, you may exile Phoenix of Iteration, then return it to the battlefield tapped under its owner's control. This ability also triggers if this card is in your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_1 { let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::W, Mana::U, Mana::B, Mana::R, Mana::G, Mana::C].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (5i64)) } {
let decider_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.state().obj(asker.source).became.is_none() && cantrip_mechanics::effects::ask_may(g, decider_2, asker.source, Purpose::Optional)? {
let objects_3 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let objects_4 = ({ let q: &dyn Query = g; ((moved_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, g.state().obj(o).owner, true)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_5.extend(moved_3.clone());
hoisted_6.extend(moved_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_8.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_9.extend(hoisted_5.clone());
hoisted_10.extend(hoisted_6.clone());
hoisted_11.extend({ let q: &dyn Query = g; hoisted_7.clone() });
hoisted_12.extend({ let q: &dyn Query = g; hoisted_8.clone() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_25: Vec<ObjectId> = Vec::new();
let mut hoisted_24: Vec<ObjectId> = Vec::new();
let mut hoisted_23: Vec<ObjectId> = Vec::new();
let mut hoisted_22: Vec<ObjectId> = Vec::new();
let mut hoisted_21: Vec<ObjectId> = Vec::new();
let mut hoisted_20: Vec<ObjectId> = Vec::new();
let mut hoisted_19: Vec<ObjectId> = Vec::new();
let mut hoisted_18: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_14: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_14 { let kind_14_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_14_0); });
g.end_sentence();
if { let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (Some(candidate).map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::W, Mana::U, Mana::B, Mana::R, Mana::G, Mana::C].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })) >= (5i64)) } {
let decider_15 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.state().obj(asker.source).became.is_none() && cantrip_mechanics::effects::ask_may(g, decider_15, asker.source, Purpose::Optional)? {
let objects_16 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_16 = g.exile_all(&objects_16);
g.end_sentence();
let objects_17 = ({ let q: &dyn Query = g; ((moved_16.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_17 = { let rows_17: Vec<(ObjectId, PlayerId, bool)> = objects_17.iter().map(|&o| (o, g.state().obj(o).owner, true)).collect(); let came_17 = g.put_all_onto_battlefield_as(&rows_17)?; came_17.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_18.extend(moved_16.clone());
hoisted_19.extend(moved_17.clone());
hoisted_20.extend({ let q: &dyn Query = g; objects_16.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_21.extend({ let q: &dyn Query = g; objects_17.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
hoisted_22.extend(hoisted_18.clone());
hoisted_23.extend(hoisted_19.clone());
hoisted_24.extend({ let q: &dyn Query = g; hoisted_20.clone() });
hoisted_25.extend({ let q: &dyn Query = g; hoisted_21.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Phoenix of Iteration", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Phoenix]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Opus — Whenever you cast an instant or sorcery spell, Phoenix of Iteration perpetually gets +1/+1. If five or more mana was spent to cast that spell, you may exile Phoenix of Iteration, then return it to the battlefield tapped under its owner's control. This ability also triggers if this card is in your graveyard.").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Opus — Whenever you cast an instant or sorcery spell, Phoenix of Iteration perpetually gets +1/+1. If five or more mana was spent to cast that spell, you may exile Phoenix of Iteration, then return it to the battlefield tapped under its owner's control. This ability also triggers if this card is in your graveyard.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
