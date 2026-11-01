// Chancellor of the Spires
// status: planned
// requires: a reveal from an opening hand
// You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, each opponent mills seven cards.
// Flying
// When this creature enters, you may cast target instant or sorcery card from an opponent's graveyard without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_5 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let mills_5: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_5)).collect(); let milled_5 = g.mill_together(&mills_5);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let objects_1 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let mut owners_1: Vec<PlayerId> = objects_1.iter().map(|o| g.state().obj(*o).owner).collect(); owners_1.dedup(); for owner in owners_1 { let cards: Vec<ObjectId> = objects_1.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
hoisted_2.extend(objects_1.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
g.delayed_trigger(DelayedWhen::Step { step: Step::Upkeep, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.state().obj(id).owner))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let caster_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_6: Vec<ObjectId> = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_6: u32 = 1; let mut played_6: Vec<ObjectId> = Vec::new();
while left_6 > 0 {
let mut open_6: Vec<ObjectId> = Vec::new(); for &c in &pool_6 { let ways = g.cast_ways_during_resolution_by(c, caster_6); if ways.iter().any(|w| true) { open_6.push(c); } }
let picked_6 = if open_6.is_empty() { None } else if open_6.len() > 1 || (false && (!false || pool_6.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_6, asker.source, Purpose::CastFree, pool_6.clone(), open_6.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_6, open_6[0], Purpose::CastFree)? { Some(open_6[0]) } else { None };
let Some(card_6) = picked_6 else { break; };
let ways_6: Vec<CastWay> = g.cast_ways_during_resolution_by(card_6, caster_6).into_iter().filter(|w| true).collect();
let done_6 = if ways_6.is_empty() { g.play_land_during_resolution(card_6)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_6, card_6, ways_6)?.map(|w| w.way); g.cast_during_resolution_as(card_6, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_6), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_6 { left_6 -= 1; played_6.push(g.state().obj(card_6).became.unwrap_or(card_6)); } else if 1i64 == 1 { break; } if done_6 || !(true && false) { pool_6.retain(|&c| c != card_6); }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("a-reveal-from-an-opening-hand" => abilities: cantrip_mechanics::opening_hand::reveal(ability_0, " "));

pub static CARD: CardDef = CardDef { name: "Chancellor of the Spires", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Sphinx]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, you may cast target instant or sorcery card from an opponent's graveyard without paying its mana cost.").targets(ABILITY_2_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, "You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, each opponent mills seven cards.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
