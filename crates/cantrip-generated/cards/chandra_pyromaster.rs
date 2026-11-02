// Chandra, Pyromaster
// status: ready
// [+1]: Chandra deals 1 damage to target player or planeswalker and 1 damage to up to one target creature that player or that planeswalker's controller controls. That creature can't block this turn.
// [0]: Exile the top card of your library. You may play it this turn.
// [−7]: Exile the top ten cards of your library. Choose an instant or sorcery card exiled this way and copy it three times. You may cast the copies without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { match t { Target::Player(p) => true, Target::Object(id) => q.is_on_battlefield(id) && { let c = q.chars(id); c.is(CardType::Creature) || c.is(CardType::Planeswalker) || c.is(CardType::Battle) } && (q.chars(id).is(CardType::Planeswalker)), _ => false } }
#[allow(unused_variables)] fn ability_0_target_1(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && q.chosen_targets().get(0).copied().and_then(|t: Target| t.player().or(t.object().map(|o| q.controller(o)))) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0),target(ability_0_target_1).between(0, 1)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let t_1_raw = announced(g, item, 1, false); let t_1 = announced(g, item, 1, true);
let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let mut batch_0: Vec<(ObjectId, Target, u16)> = Vec::new();
let amount_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; if let Some(source) = source_0 { for t in { let q: &dyn Query = g; t_0.clone() } { batch_0.push((source, t, amount_0_0)); } }
let amount_0_1 = { let q: &dyn Query = g; (1i64).max(0) as u16 }; if let Some(source) = source_0 { for t in { let q: &dyn Query = g; (t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { batch_0.push((source, t, amount_0_1)); } }
g.damage_together(&batch_0);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (t_1_raw.get(0).copied().filter(|t| t_1.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::CantBlock };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
cantrip_api::instruction(g, |g| { let cards_3_0 = { let q: &dyn Query = g; (moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_3_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_3_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_3_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::EndOfTurn, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((10i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_5 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && ((moved_4.clone()).contains(&id) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)))).collect::<Vec<ObjectId>>() }; let times_5 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mut copies_5: Vec<ObjectId> = Vec::new(); for _ in 0..times_5 { for &card in &originals_5 { copies_5.push(g.create_copy_in_exile(card, player_5)); } }
g.end_sentence();
let caster_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_6: Vec<ObjectId> = { let q: &dyn Query = g; (copies_5.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_6: u32 = 4294967295; let mut played_6: Vec<ObjectId> = Vec::new();
while left_6 > 0 {
let mut open_6: Vec<ObjectId> = Vec::new(); for &c in &pool_6 { let ways = g.cast_ways_during_resolution_by(c, caster_6); if ways.iter().any(|w| true) { open_6.push(c); } }
let picked_6 = if open_6.is_empty() { None } else if open_6.len() > 1 || (true && (!true || pool_6.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_6, asker.source, Purpose::CastFree, pool_6.clone(), open_6.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_6, open_6[0], Purpose::CastFree)? { Some(open_6[0]) } else { None };
let Some(card_6) = picked_6 else { break; };
let ways_6: Vec<CastWay> = g.cast_ways_during_resolution_by(card_6, caster_6).into_iter().filter(|w| true).collect();
let done_6 = { let way = cantrip_mechanics::choose::cast_way(g, caster_6, card_6, ways_6)?.map(|w| w.way); g.cast_during_resolution_as(card_6, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_6), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_6 { left_6 -= 1; played_6.push(g.state().obj(card_6).became.unwrap_or(card_6)); } if done_6 || !true { pool_6.retain(|&c| c != card_6); }
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Chandra, Pyromaster", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Chandra]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(4), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_0, "+1: Chandra deals 1 damage to target player or planeswalker and 1 damage to up to one target creature that player or that planeswalker's controller controls. That creature can't block this turn.").targets(ABILITY_0_TARGETS).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(0)), ability_1, "0: Exile the top card of your library. You may play it this turn.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-7)), ability_2, "−7: Exile the top ten cards of your library. Choose an instant or sorcery card exiled this way and copy it three times. You may cast the copies without paying their mana costs.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
