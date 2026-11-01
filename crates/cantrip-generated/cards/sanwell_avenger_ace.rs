// Sanwell, Avenger Ace
// status: ready
// As long as an artifact creature you control is attacking, prevent all damage that would be dealt to Sanwell.
// Whenever Sanwell becomes tapped, exile the top six cards of your library. You may cast a Vehicle or artifact creature spell from among them. Then put the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) && (((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64) { return true; } }
 false }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Tapped { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((6i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
let caster_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_1: Vec<ObjectId> = { let q: &dyn Query = g; ((moved_0.clone()).into_iter().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle)) || (q.chars(id).is(CardType::Artifact) && q.chars(id).is(CardType::Creature)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_1: u32 = 1; let mut played_1: Vec<ObjectId> = Vec::new();
while left_1 > 0 {
let mut open_1: Vec<ObjectId> = Vec::new(); for &c in &pool_1 { let ways = g.cast_ways_during_resolution_by(c, caster_1); if ways.iter().any(|w| true) { open_1.push(c); } }
let picked_1 = if open_1.is_empty() { None } else if open_1.len() > 1 || (true && (!false || pool_1.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_1, asker.source, Purpose::Optional, pool_1.clone(), open_1.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_1, open_1[0], Purpose::Optional)? { Some(open_1[0]) } else { None };
let Some(card_1) = picked_1 else { break; };
let ways_1: Vec<CastWay> = g.cast_ways_during_resolution_by(card_1, caster_1).into_iter().filter(|w| true).collect();
let done_1 = if ways_1.is_empty() { g.play_land_during_resolution(card_1)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_1, card_1, ways_1)?.map(|w| w.way); g.cast_during_resolution_as(card_1, CastTerms { instead: None, way, caster: Some(caster_1), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_1 { left_1 -= 1; played_1.push(g.state().obj(card_1).became.unwrap_or(card_1)); } else if 1i64 == 1 { break; } if done_1 || !(true && true) { pool_1.retain(|&c| c != card_1); }
}
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (moved_0.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }; let moved_2 = g.put_into_library(&objects_2, LibraryPlace::BottomInRandomOrder);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sanwell, Avenger Ace", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Pilot]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::TAPPED.0), ability_1_event), ability_1, "Whenever Sanwell becomes tapped, exile the top six cards of your library. You may cast a Vehicle or artifact creature spell from among them. Then put the rest on the bottom of your library in a random order.").targets(&[])], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "As long as an artifact creature you control is attacking, prevent all damage that would be dealt to Sanwell.", applies: ability_0_0_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
