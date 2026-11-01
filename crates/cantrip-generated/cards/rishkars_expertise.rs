// Rishkar's Expertise
// status: ready
// Draw cards equal to the greatest power among creatures you control.
// You may cast a spell with mana value 5 or less from your hand without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))).max().unwrap_or(0)).clamp(0, 4294967295) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let caster_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_1: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_1: u32 = 1; let mut played_1: Vec<ObjectId> = Vec::new();
let limit_1_0: i64 = i64::from({ let q: &dyn Query = g; 5i64 });
while left_1 > 0 {
let mut open_1: Vec<ObjectId> = Vec::new(); for &c in &pool_1 { let ways = g.cast_ways_during_resolution_by(c, caster_1); if ways.iter().any(|w| (i64::from(w.mana_value)) <= (limit_1_0)) { open_1.push(c); } }
let picked_1 = if open_1.is_empty() { None } else if open_1.len() > 1 || (true && (!false || pool_1.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_1, asker.source, Purpose::CastFree, pool_1.clone(), open_1.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_1, open_1[0], Purpose::CastFree)? { Some(open_1[0]) } else { None };
let Some(card_1) = picked_1 else { break; };
let ways_1: Vec<CastWay> = g.cast_ways_during_resolution_by(card_1, caster_1).into_iter().filter(|w| (i64::from(w.mana_value)) <= (limit_1_0)).collect();
let done_1 = if ways_1.is_empty() { g.play_land_during_resolution(card_1)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_1, card_1, ways_1)?.map(|w| w.way); g.cast_during_resolution_as(card_1, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_1), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_1 { left_1 -= 1; played_1.push(g.state().obj(card_1).became.unwrap_or(card_1)); } else if 1i64 == 1 { break; } if done_1 || !(true && true) { pool_1.retain(|&c| c != card_1); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rishkar's Expertise", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
