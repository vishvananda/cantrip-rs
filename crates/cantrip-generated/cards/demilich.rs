// Demilich
// status: planned
// requires: engine: a card's own terms to cast it from its graveyard
// This spell costs {U} less to cast for each instant and sorcery spell you've cast this turn.
// Whenever this creature attacks, exile up to one target instant or sorcery card from your graveyard. Copy it. You may cast the copy.
// You may cast this card from your graveyard by exiling four instant and/or sorcery cards from your graveyard in addition to paying its other costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && true { { let mut less = ManaCost::ZERO; less.symbols[1] += (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }  None }).count() as i64).clamp(0, 255) as u8; CostDelta::less_mana(less) } } else { CostDelta::NONE } }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_1 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_1: Vec<ObjectId> = Vec::new(); for _ in 0..times_1 { for &card in &originals_1 { copies_1.push(g.create_copy_in_exile(card, player_1)); } }
g.end_sentence();
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (copies_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| true) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (true && (!true || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::Optional, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::Optional)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| true).collect();
let done_2 = if ways_2.is_empty() { g.play_land_during_resolution(card_2)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: None, way, caster: Some(caster_2), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else if true { break; } if done_2 || !(true && true) { pool_2.retain(|&c| c != card_2); }
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_3(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-a-card-s-own-terms-to-cast-it-from-its-graveyard" => ways: way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 4, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(exile_from_graveyard_of(4, ability_2_3)), ZoneKind::Graveyard, "Cast from your graveyard").only_from());

pub static CARD: CardDef = CardDef { name: "Demilich", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 4, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Skeleton,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, exile up to one target instant or sorcery card from your graveyard. Copy it. You may cast the copy.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
