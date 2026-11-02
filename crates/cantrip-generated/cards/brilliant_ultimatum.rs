// Brilliant Ultimatum
// status: ready
// Exile the top five cards of your library. An opponent separates those cards into two piles. You may play lands and cast spells from one of those piles. If you cast a spell this way, you cast it without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((5i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
g.end_sentence();
let among: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let Some(picked_0) = (if !among.is_empty() { cantrip_mechanics::choose::player(g, asker.controller, asker.source, among, "ChooseRevealedCard", 0)? } else { among.first().copied() }) else { return Ok(()); };
let cards_2 = { let q: &dyn Query = g; ((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_0.clone()).contains(&id)).collect::<Vec<ObjectId>>() }; let separator_2 = { let q: &dyn Query = g; Some(picked_0).expect("a reference that always names a player") }; let chooser_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (first_2, second_2) = cantrip_mechanics::choose::separate(g, Some(separator_2), asker.source, cards_2)?; let (pile_2, other_2) = cantrip_mechanics::choose::pile(g, chooser_2, asker.source, first_2, second_2)?;

g.end_sentence();
let caster_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_3: Vec<ObjectId> = { let q: &dyn Query = g; (pile_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_3: u32 = 4294967295; let mut played_3: Vec<ObjectId> = Vec::new();
while left_3 > 0 {
let mut open_3: Vec<ObjectId> = Vec::new(); for &c in &pool_3 { let ways = g.cast_ways_during_resolution_by(c, caster_3); if ways.iter().any(|w| true) || g.could_play_land_during_resolution(c) { open_3.push(c); } }
let picked_3 = if open_3.is_empty() { None } else if open_3.len() > 1 || (true && (!false || pool_3.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_3, asker.source, Purpose::CastFree, pool_3.clone(), open_3.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_3, open_3[0], Purpose::CastFree)? { Some(open_3[0]) } else { None };
let Some(card_3) = picked_3 else { break; };
let ways_3: Vec<CastWay> = g.cast_ways_during_resolution_by(card_3, caster_3).into_iter().filter(|w| true).collect();
let done_3 = if ways_3.is_empty() { g.play_land_during_resolution(card_3)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_3, card_3, ways_3)?.map(|w| w.way); g.cast_during_resolution_as(card_3, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_3), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_3 { left_3 -= 1; played_3.push(g.state().obj(card_3).became.unwrap_or(card_3)); } else if false { break; } if done_3 || !(true && true) { pool_3.retain(|&c| c != card_3); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Brilliant Ultimatum", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 3, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
