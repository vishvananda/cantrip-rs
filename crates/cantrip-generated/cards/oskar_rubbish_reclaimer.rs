// Oskar, Rubbish Reclaimer
// status: ready
// This spell costs {1} less to cast for each different mana value among cards in your graveyard.
// Whenever you discard a nonland card, you may cast it from your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && true { CostDelta::less(({ let mut seen = Vec::new(); for id in ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() { let value = q.mana_value(id); if !seen.contains(&value) { seen.push(value); } } seen.len() as i64 }).clamp(0, 255) as u8) } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && !(q.chars(id).is(CardType::Land)) && !*undefined { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let caster_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (((item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = 1; let mut played_0: Vec<ObjectId> = Vec::new();
while left_0 > 0 {
let mut open_0: Vec<ObjectId> = Vec::new(); for &c in &pool_0 { let ways = g.cast_ways_during_resolution_by(c, caster_0); if ways.iter().any(|w| true) { open_0.push(c); } }
let picked_0 = if open_0.is_empty() { None } else if open_0.len() > 1 || (false && (!false || pool_0.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_0, asker.source, Purpose::Optional, pool_0.clone(), open_0.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_0, open_0[0], Purpose::Optional)? { Some(open_0[0]) } else { None };
let Some(card_0) = picked_0 else { break; };
let ways_0: Vec<CastWay> = g.cast_ways_during_resolution_by(card_0, caster_0).into_iter().filter(|w| true).collect();
let done_0 = if ways_0.is_empty() { g.play_land_during_resolution(card_0)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_0, card_0, ways_0)?.map(|w| w.way); g.cast_during_resolution_as(card_0, CastTerms { instead: None, way, caster: Some(caster_0), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_0 { left_0 -= 1; played_0.push(g.state().obj(card_0).became.unwrap_or(card_0)); } else if 1i64 == 1 { break; } if done_0 || !(true && false) { pool_0.retain(|&c| c != card_0); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Oskar, Rubbish Reclaimer", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DISCARDED.0), ability_1_event), ability_1, "Whenever you discard a nonland card, you may cast it from your graveyard.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
