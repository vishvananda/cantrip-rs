// Bruna, Light of Alabaster
// status: planned
// requires: several objects arriving attached
// Flying, vigilance
// Whenever Bruna attacks or blocks, you may attach to it any number of Auras on the battlefield and you may put onto the battlefield attached to it any number of Aura cards that could enchant it from your graveyard and/or hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let eligible_2 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_2 = { let q: &dyn Query = g; item.subject.object() }; let eligible_2 = if host_2.is_some() { eligible_2 } else { Vec::new() }; let count_2 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let chosen_2 = cantrip_mechanics::choose::select(g, player_2, asker.source, Purpose::Attach, eligible_2, cantrip_mechanics::select::Selection::of(count_2, &[]))?; if let Some(host) = host_2 { cantrip_api::instruction(g, |g| { for &attachment in &chosen_2 { g.attach(attachment, host); } }); }
g.end_sentence();
hoisted_3.extend(chosen_2.clone());
}
g.end_sentence();
let decider_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_4, asker.source, Purpose::Optional)? {
let selected_6_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::PutOntoBattlefield, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
let objects_7 = ({ let q: &dyn Query = g; selected_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = planned_value!("several objects arriving attached", Vec::<ObjectId>::new() => { let rows_7: Vec<(ObjectId, PlayerId)> = objects_7.iter().map(|&o| (o, g.state().obj(o).owner)).collect(); g.put_all_onto_battlefield_attached(&rows_7, { let q: &dyn Query = g; item.subject.object() })? });
g.end_sentence();
hoisted_8.extend(moved_7.clone());
hoisted_9.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_19: Vec<ObjectId> = Vec::new();
let mut hoisted_18: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_10 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_10, asker.source, Purpose::Optional)? {
let player_12 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let eligible_12 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_12 = { let q: &dyn Query = g; item.subject.object() }; let eligible_12 = if host_12.is_some() { eligible_12 } else { Vec::new() }; let count_12 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let chosen_12 = cantrip_mechanics::choose::select(g, player_12, asker.source, Purpose::Attach, eligible_12, cantrip_mechanics::select::Selection::of(count_12, &[]))?; if let Some(host) = host_12 { cantrip_api::instruction(g, |g| { for &attachment in &chosen_12 { g.attach(attachment, host); } }); }
g.end_sentence();
hoisted_13.extend(chosen_12.clone());
}
g.end_sentence();
let decider_14 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_14, asker.source, Purpose::Optional)? {
let selected_16_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_16_candidates = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura))).collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } all } }; let selected_16_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AnyNumber }; let selected_16 = { cantrip_mechanics::choose::select(g, selected_16_player, asker.source, Purpose::PutOntoBattlefield, selected_16_candidates, cantrip_mechanics::select::Selection::of(selected_16_count, &[]))? };
let objects_17 = ({ let q: &dyn Query = g; selected_16.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_17 = planned_value!("several objects arriving attached", Vec::<ObjectId>::new() => { let rows_17: Vec<(ObjectId, PlayerId)> = objects_17.iter().map(|&o| (o, g.state().obj(o).owner)).collect(); g.put_all_onto_battlefield_attached(&rows_17, { let q: &dyn Query = g; item.subject.object() })? });
g.end_sentence();
hoisted_18.extend(moved_17.clone());
hoisted_19.extend({ let q: &dyn Query = g; objects_17.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bruna, Light of Alabaster", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever Bruna attacks or blocks, you may attach to it any number of Auras on the battlefield and you may put onto the battlefield attached to it any number of Aura cards that could enchant it from your graveyard and/or hand.").targets(&[]),triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_3_event), ability_3, "Whenever Bruna attacks or blocks, you may attach to it any number of Auras on the battlefield and you may put onto the battlefield attached to it any number of Aura cards that could enchant it from your graveyard and/or hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
