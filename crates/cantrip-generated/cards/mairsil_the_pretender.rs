// Mairsil, the Pretender
// status: ready
// When Mairsil enters, you may exile an artifact or creature card from your hand or graveyard and put a cage counter on it.
// Mairsil has all activated abilities of all cards you own in exile with cage counters on them. You may activate each of those abilities only once each turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let offered_0: Vec<u8> = [(0u8, !{ let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }.is_empty()), (1u8, !{ let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }.is_empty()), (2u8, true)].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_0 = item.ability.card; let picked_0 = if offered_0.len() > 1 { offered_0[cantrip_mechanics::choose::one_of(g, decider_0, asker.source, offered_0.iter().map(|&option| Label::Card(card_0, option)).collect())? as usize] } else { offered_0.first().copied().unwrap_or(u8::MAX) }; match picked_0 {
0 => {
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Exile, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; ((moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_3.push((object, CounterKind::Cage, amount)); } }
let had_3: Vec<_> = counters_3.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_3); let applied_3: Vec<ObjectId> = counters_3.iter().zip(had_3).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_4.extend(moved_2.clone());
hoisted_5.extend(selected_1.clone());
hoisted_6.extend(applied_3.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_2.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
1 => {
let selected_8_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_8_candidates = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let selected_8_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_8 = { cantrip_mechanics::choose::select(g, selected_8_player, asker.source, Purpose::Exile, selected_8_candidates, cantrip_mechanics::select::Selection::of(selected_8_count, &[]))? };
let objects_9 = { let q: &dyn Query = g; selected_8.clone() }; let moved_9 = g.exile_all(&objects_9);
g.end_sentence();
let mut counters_10: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; ((moved_9.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_10.push((object, CounterKind::Cage, amount)); } }
let had_10: Vec<_> = counters_10.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_10); let applied_10: Vec<ObjectId> = counters_10.iter().zip(had_10).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
hoisted_11.extend(moved_9.clone());
hoisted_12.extend(selected_8.clone());
hoisted_13.extend(applied_10.clone());
hoisted_14.extend({ let q: &dyn Query = g; objects_9.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
2 => {
g.end_sentence();
}
_ => {}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let cards: Vec<CardId> = (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (Some(asker.controller) == Some(q.state().obj(id).owner) && (i64::from(q.state().obj(id).counters(CounterKind::Cage))) > (0i64))).collect::<Vec<ObjectId>>()).into_iter().map(|id: ObjectId| q.state().obj(id).card).collect(); cantrip_mechanics::statics::grant_activated_abilities_of(q, &cards, chars); }
#[allow(unused_variables)] fn granted_once(q: &dyn Query, source: ObjectId, ability: AbilityRef) -> usize { let asker = &Asker::new(source, q.controller(source)); let ids: Vec<ObjectId> = ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (Some(asker.controller) == Some(q.state().obj(id).owner) && (i64::from(q.state().obj(id).counters(CounterKind::Cage))) > (0i64))).collect::<Vec<ObjectId>>(); ids.into_iter().filter(|&id| q.state().obj(id).card == ability.card).count() }

pub static CARD: CardDef = CardDef { name: "Mairsil, the Pretender", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Mairsil enters, you may exile an artifact or creature card from your hand or graveyard and put a cage counter on it.").targets(&[])], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_1_apply_0, "Mairsil has all activated abilities of all cards you own in exile with cage counters on them. You may activate each of those abilities only once each turn.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_once_each_turn: Some(granted_once), labels: &["Hand", "Graveyard", "Neither"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
