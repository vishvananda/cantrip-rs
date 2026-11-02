// Sproutback Trudge
// status: ready
// This spell costs {X} less to cast, where X is the amount of life you gained this turn.
// Trample
// At the beginning of your end step, if you gained life this turn, you may cast this creature from your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_cost(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, player); if costed == Costed::Spell(source) && true { CostDelta::less((q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()).clamp(0, 255) as u8) } else { CostDelta::NONE } }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }  None }).count() as i64) >= (1i64) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let caster_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = 1; let mut played_0: Vec<ObjectId> = Vec::new();
while left_0 > 0 {
let mut open_0: Vec<ObjectId> = Vec::new(); for &c in &pool_0 { let ways = g.cast_ways_during_resolution_by(c, caster_0); if ways.iter().any(|w| true) { open_0.push(c); } }
let picked_0 = if open_0.is_empty() { None } else if open_0.len() > 1 || (false && (!false || pool_0.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_0, asker.source, Purpose::Optional, pool_0.clone(), open_0.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_0, open_0[0], Purpose::Optional)? { Some(open_0[0]) } else { None };
let Some(card_0) = picked_0 else { break; };
let ways_0: Vec<CastWay> = g.cast_ways_during_resolution_by(card_0, caster_0).into_iter().filter(|w| true).collect();
let done_0 = if ways_0.is_empty() { g.play_land_during_resolution(card_0)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_0, card_0, ways_0)?.map(|w| w.way); g.cast_during_resolution_as(card_0, CastTerms { instead: None, way, caster: Some(caster_0), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_0 { left_0 -= 1; played_0.push(g.state().obj(card_0).became.unwrap_or(card_0)); } else if true { break; } if done_0 || !(true && false) { pool_0.retain(|&c| c != card_0); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sproutback Trudge", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Fungus,Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(9), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of your end step, if you gained life this turn, you may cast this creature from your graveyard.").targets(&[]).condition(ability_2_if).functions_in(ZoneKinds::of(&[ZoneKind::Graveyard]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_cost], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
