// Kess, Dissident Mage
// status: planned
// requires: player rule SpecialAction
// Flying
// Once during each of your turns, you may cast an instant or sorcery spell from your graveyard. If a spell cast this way would be put into your graveyard, exile it instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-special-action" => statics: player_rule(ability_1_0_applies, PlayerRule::SpecialAction { body: |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let caster_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_0: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = 1; let mut played_0: Vec<ObjectId> = Vec::new();
while left_0 > 0 {
let mut open_0: Vec<ObjectId> = Vec::new(); for &c in &pool_0 { let ways = g.cast_ways_during_resolution_by(c, caster_0); if ways.iter().any(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))) { open_0.push(c); } }
let picked_0 = if open_0.is_empty() { None } else if open_0.len() > 1 || (true && (!false || pool_0.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_0, asker.source, Purpose::Optional, pool_0.clone(), open_0.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_0, open_0[0], Purpose::Optional)? { Some(open_0[0]) } else { None };
let Some(card_0) = picked_0 else { break; };
let ways_0: Vec<CastWay> = g.cast_ways_during_resolution_by(card_0, caster_0).into_iter().filter(|w| (g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery))).collect();
let done_0 = { let way = cantrip_mechanics::choose::cast_way(g, caster_0, card_0, ways_0)?.map(|w| w.way); g.cast_during_resolution_as(card_0, CastTerms { instead: None, way, caster: Some(caster_0), exile_instead: ExileInstead::Spell, ..CastTerms::default() })? };
if done_0 { left_0 -= 1; played_0.push(g.state().obj(card_0).became.unwrap_or(card_0)); } else { break; } if done_0 || !true { pool_0.retain(|&c| c != card_0); }
}
g.end_sentence();
 Ok(()) }, cost: None, limit: Some(PlayLimit::EachTurn(1)), during: Some(|q: &dyn Query, asker: &Asker, player: PlayerId| player == asker.controller), instant_speed: false }));

pub static CARD: CardDef = CardDef { name: "Kess, Dissident Mage", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
