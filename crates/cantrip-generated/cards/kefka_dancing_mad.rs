// Kefka, Dancing Mad
// status: ready
// During your turn, Kefka has indestructible.
// At the beginning of your end step, exile a card at random from each opponent's graveyard. You may cast any number of spells from among cards exiled this way without paying their mana costs. Then each player who owns a spell you cast this way loses life equal to its mana value.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (Some(asker.controller) == Some(q.state().turn.active)) && (source == affected) }
#[allow(unused_variables)] fn ability_0_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Indestructible); }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut pool_1 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (true && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(bound_0) == Some(q.state().obj(id).owner)))).collect::<Vec<ObjectId>>() }; let count_1 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_1: Vec<ObjectId> = Vec::new(); while picked_1.len() < count_1 && !pool_1.is_empty() { let at = g.choose_at_random(pool_1.len()); picked_1.push(pool_1.remove(at)); }
 each_2[index_0] = picked_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; all_2.clone() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let caster_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_4: Vec<ObjectId> = { let q: &dyn Query = g; (moved_3.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_4: u32 = 4294967295; let mut played_4: Vec<ObjectId> = Vec::new();
while left_4 > 0 {
let mut open_4: Vec<ObjectId> = Vec::new(); for &c in &pool_4 { let ways = g.cast_ways_during_resolution_by(c, caster_4); if ways.iter().any(|w| true) { open_4.push(c); } }
let picked_4 = if open_4.is_empty() { None } else if open_4.len() > 1 || (false && (!false || pool_4.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_4, asker.source, Purpose::CastFree, pool_4.clone(), open_4.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_4, open_4[0], Purpose::CastFree)? { Some(open_4[0]) } else { None };
let Some(card_4) = picked_4 else { break; };
let ways_4: Vec<CastWay> = g.cast_ways_during_resolution_by(card_4, caster_4).into_iter().filter(|w| true).collect();
let done_4 = if ways_4.is_empty() { g.play_land_during_resolution(card_4)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_4, card_4, ways_4)?.map(|w| w.way); g.cast_during_resolution_as(card_4, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_4), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_4 { left_4 -= 1; played_4.push(g.state().obj(card_4).became.unwrap_or(card_4)); } else if false { break; } if done_4 || !(true && false) { pool_4.retain(|&c| c != card_4); }
}
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| (((q.state().zone(Zone::STACK).to_vec()).into_iter().filter(|&id| true && ((played_4.clone()).contains(&id) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64)).collect::<Vec<PlayerId>>() };
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { let amount_6 = { let q: &dyn Query = g; (((q.state().zone(Zone::STACK).to_vec()).into_iter().filter(|&id| true && ((played_4.clone()).contains(&id) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).sum::<i64>()).clamp(0, 2147483647) as i32 }; let players_6 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.lose_life(player, amount_6); }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Kefka, Dancing Mad", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, exile a card at random from each opponent's graveyard. You may cast any number of spells from among cards exiled this way without paying their mana costs. Then each player who owns a spell you cast this way loses life equal to its mana value.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_0_0_affects, ability_0_0_apply_0, "During your turn, Kefka has indestructible.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
