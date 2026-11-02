// Lurking Spinecrawler
// status: planned
// requires: engine: no perpetual effects
// When this creature enters, you may choose a nonland card in your hand. If you do, that card perpetually incorporates {1}{B} and gains "When you cast this spell, each opponent sacrifices a creature of their choice. Each opponent who can't discards a card."
// Whenever you cast a multicolored spell, for each of that spell's colors, each opponent loses 1 life and you gain 1 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn perpetual_5_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_5(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_6 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }; let mut picks_6: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_6 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_6 { let eligible: Vec<ObjectId> = objects_6.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_6.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_6.clone(), &[]))? }; for object in taken { picks_6.push((player, object)); } }
let sacrificed_6 = g.sacrifice_each(&picks_6);
g.end_sentence();
let amount_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_7: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !((((picks_6.iter().filter(|(who, o)| Some(*who) == Some(bound_0) && q.state().obj(*o).became.is_some()).map(|(_, o)| *o).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>()).len() as i64) > (0i64))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } }.into_iter().collect(); let mut cards_7 = Vec::new(); for player in g.state().apnap().filter(|p| players_7.contains(p)).collect::<Vec<PlayerId>>() { cards_7.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_7)?); } let discarded_7 = g.discard_together(&cards_7);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut paid_0 = false; {
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
if !selected_1.is_empty() { paid_0 = true;
g.end_sentence();
hoisted_2.extend(selected_1.clone());
}
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("engine-no-perpetual-effects" => let objects_4: Vec<ObjectId> = { let q: &dyn Query = g; (hoisted_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_4_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Incorporate(ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }) };
let kind_4_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) };
for &object in &objects_4 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_0); cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_4_1); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).colors.iter().count() as i64) > (1i64) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_8 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| q.chars(o).colors.iter().count() as i64)).max(0) as i32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_8 { g.lose_life(player, amount_8); }
g.end_sentence();
let amount_9 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| q.chars(o).colors.iter().count() as i64)).max(0) as i32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.gain_life(player, amount_9); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Lurking Spinecrawler", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you may choose a nonland card in your hand. If you do, that card perpetually incorporates {1}{B} and gains \"When you cast this spell, each opponent sacrifices a creature of their choice. Each opponent who can't discards a card.\"").targets(&[]),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever you cast a multicolored spell, for each of that spell's colors, each opponent loses 1 life and you gain 1 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), perpetual_5_event), perpetual_5, "When you cast this spell, each opponent sacrifices a creature of their choice. Each opponent who can't discards a card.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack]))], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
