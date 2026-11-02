// Mapping the Maze
// status: planned
// requires: engine: no perpetual effects
// Choose an instant or sorcery card in your hand or graveyard. That card perpetually incorporates {U} and gains "When you cast this spell, draw a card." Then return a multicolored instant or sorcery card from your graveyard to your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn perpetual_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn perpetual_10_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_10(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_11 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_11 { g.draw(player, amount_11); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let offered_0: Vec<u8> = [(0u8, !{ let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }.is_empty()), (1u8, !{ let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }.is_empty())].into_iter().filter(|option| option.1).map(|option| option.0).collect(); let card_0 = item.ability.card; let picked_0 = if offered_0.len() > 1 { offered_0[cantrip_mechanics::choose::one_of(g, decider_0, asker.source, offered_0.iter().map(|&option| Label::Card(card_0, option)).collect())? as usize] } else { offered_0.first().copied().unwrap_or(u8::MAX) }; match picked_0 {
0 => {
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; (selected_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_3 { let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Incorporate(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); let kind_3_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_1); });
g.end_sentence();
hoisted_6.extend(selected_1.clone());
}
1 => {
let selected_7_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_7_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>() }; let selected_7_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_7 = { cantrip_mechanics::choose::select(g, selected_7_player, asker.source, Purpose::ChooseAsEnters, selected_7_candidates, cantrip_mechanics::select::Selection::of(selected_7_count, &[]))? };
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_9: Vec<ObjectId> = { let q: &dyn Query = g; (selected_7.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_9 { let kind_9_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Incorporate(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_9_0); let kind_9_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_9_1); });
g.end_sentence();
hoisted_12.extend(selected_7.clone());
}
_ => {}
}
g.end_sentence();
let selected_13_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_13_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((q.chars(id).colors.iter().count() as i64) > (1i64) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))).collect::<Vec<ObjectId>>() }; let selected_13_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_13 = { cantrip_mechanics::choose::select(g, selected_13_player, asker.source, Purpose::PutIntoHand, selected_13_candidates, cantrip_mechanics::select::Selection::of(selected_13_count, &[]))? };
let objects_14 = { let q: &dyn Query = g; selected_13.clone() }; let moved_14 = g.return_to_hand(&objects_14);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mapping the Maze", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), perpetual_4_event), perpetual_4, "When you cast this spell, draw a card.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack])),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), perpetual_10_event), perpetual_10, "When you cast this spell, draw a card.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack]))], enters: None, modal: None, enchant: None, labels: &["Hand", "Graveyard"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
