// Elite Arcanist
// status: planned
// requires: engine: no rule on an ability's X
// When this creature enters, you may exile an instant card from your hand.
// {X}, {T}: Copy the exiled card. You may cast the copy without paying its mana cost. X is the mana value of the exiled card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Instant)).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Exile, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
if !selected_0.is_empty() { 
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let moved_1 = { let by = g.linked(item); g.exile_with(&objects_1, by, false) };
g.end_sentence();
hoisted_2.extend(moved_1.clone());
hoisted_3.extend(selected_0.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_5 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_5: Vec<ObjectId> = Vec::new(); for _ in 0..times_5 { for &card in &originals_5 { copies_5.push(g.create_copy_in_exile(card, player_5)); } }
g.end_sentence();
let caster_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_6: Vec<ObjectId> = { let q: &dyn Query = g; (copies_5.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_6: u32 = 1; let mut played_6: Vec<ObjectId> = Vec::new();
while left_6 > 0 {
let mut open_6: Vec<ObjectId> = Vec::new(); for &c in &pool_6 { let ways = g.cast_ways_during_resolution_by(c, caster_6); if ways.iter().any(|w| true) { open_6.push(c); } }
let picked_6 = if open_6.is_empty() { None } else if open_6.len() > 1 || (true && (!true || pool_6.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_6, asker.source, Purpose::CastFree, pool_6.clone(), open_6.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_6, open_6[0], Purpose::CastFree)? { Some(open_6[0]) } else { None };
let Some(card_6) = picked_6 else { break; };
let ways_6: Vec<CastWay> = g.cast_ways_during_resolution_by(card_6, caster_6).into_iter().filter(|w| true).collect();
let done_6 = { let way = cantrip_mechanics::choose::cast_way(g, caster_6, card_6, ways_6)?.map(|w| w.way); g.cast_during_resolution_as(card_6, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_6), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_6 { left_6 -= 1; played_6.push(g.state().obj(card_6).became.unwrap_or(card_6)); } else { break; } if done_6 || !true { pool_6.retain(|&c| c != card_6); }
}
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-rule-on-an-ability-s-x" => #[allow(unused_variables)] fn ability_1_x_is(q: &dyn Query, asker: &Asker) -> i64 { (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o))) });

pub static CARD: CardDef = CardDef { name: "Elite Arcanist", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, you may exile an instant card from your hand.").targets(&[]),planned_value!("engine: no rule on an ability's X", ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{X}, {T}: Copy the exiled card. You may cast the copy without paying its mana cost. X is the mana value of the exiled card.").targets(&[]) => ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{X}, {T}: Copy the exiled card. You may cast the copy without paying its mana cost. X is the mana value of the exiled card.").targets(&[]).x_is(ability_1_x_is))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
