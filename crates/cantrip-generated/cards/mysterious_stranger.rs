// Mysterious Stranger
// status: ready
// Flash
// When this creature enters, for each graveyard with an instant or sorcery card in it, exile target instant or sorcery card from that graveyard. If two or more cards are exiled this way, choose one of them at random and copy it. You may cast the copy without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn different_owners_key(q: &dyn Query, id: ObjectId) -> PlayerId { q.state().obj(id).owner }
fn different_owners(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let used: Vec<PlayerId> = choice.selected.iter().map(|&c| different_owners_key(q, c)).collect(); let apart = used.iter().enumerate().all(|(i, k)| !used[..i].contains(k)); Verdict { complete: apart, addable: Bits::of(choice.candidates.len(), |i| !used.contains(&different_owners_key(q, choice.candidates[i]))) } }
fn largest_different_owners(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { cantrip_api::select::largest_distinct(choice, |c| different_owners_key(q, c)) }
#[allow(unused_variables)] fn ability_1_0_count(q: &dyn Query, asker: &Asker) -> u16 { ((q.state().apnap().filter(|&bound_0: &PlayerId| ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64)).collect::<Vec<PlayerId>>()).len() as i64).clamp(0, 65535) as u16 }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().apnap().filter(|&bound_0: &PlayerId| ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64)).collect::<Vec<PlayerId>>()).contains(&q.state().obj(id).owner)))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).counted().together(cantrip_api::select::set_part(&(different_owners as SelectFn), &(largest_different_owners as cantrip_api::select::LargestFn), 0, "with different owners"))];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
if { let q: &dyn Query = g; (((moved_0.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>()).len() as i64) >= (2i64) } {
let mut pool_1 = { let q: &dyn Query = g; (moved_0.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_1: Vec<ObjectId> = Vec::new(); while picked_1.len() < count_1 && !pool_1.is_empty() { let at = g.choose_at_random(pool_1.len()); picked_1.push(pool_1.remove(at)); }
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_2 = { let q: &dyn Query = g; (picked_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_2: Vec<ObjectId> = Vec::new(); for _ in 0..times_2 { for &card in &originals_2 { copies_2.push(g.create_copy_in_exile(card, player_2)); } }
g.end_sentence();
let caster_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_3: Vec<ObjectId> = { let q: &dyn Query = g; (copies_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_3: u32 = 1; let mut played_3: Vec<ObjectId> = Vec::new();
while left_3 > 0 {
let mut open_3: Vec<ObjectId> = Vec::new(); for &c in &pool_3 { let ways = g.cast_ways_during_resolution_by(c, caster_3); if ways.iter().any(|w| true) { open_3.push(c); } }
let picked_3 = if open_3.is_empty() { None } else if open_3.len() > 1 || (true && (!true || pool_3.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_3, asker.source, Purpose::CastFree, pool_3.clone(), open_3.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_3, open_3[0], Purpose::CastFree)? { Some(open_3[0]) } else { None };
let Some(card_3) = picked_3 else { break; };
let ways_3: Vec<CastWay> = g.cast_ways_during_resolution_by(card_3, caster_3).into_iter().filter(|w| true).collect();
let done_3 = if ways_3.is_empty() { g.play_land_during_resolution(card_3)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_3, card_3, ways_3)?.map(|w| w.way); g.cast_during_resolution_as(card_3, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_3), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_3 { left_3 -= 1; played_3.push(g.state().obj(card_3).became.unwrap_or(card_3)); } else if true { break; } if done_3 || !(true && true) { pool_3.retain(|&c| c != card_3); }
}
g.end_sentence();
hoisted_4.extend(picked_1.clone());
hoisted_5.extend(copies_2.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Mysterious Stranger", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, for each graveyard with an instant or sorcery card in it, exile target instant or sorcery card from that graveyard. If two or more cards are exiled this way, choose one of them at random and copy it. You may cast the copy without paying its mana cost.").targets(ABILITY_1_TARGETS).counted_by(ability_1_0_count)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
