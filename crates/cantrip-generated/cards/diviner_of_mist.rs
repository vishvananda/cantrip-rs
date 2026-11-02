// Diviner of Mist
// status: ready
// Flying
// Whenever this creature attacks, mill four cards. You may cast an instant or sorcery spell from your graveyard with mana value 4 or less without paying its mana cost. If that spell would be put into your graveyard, exile it instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let caster_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_1: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_1: u32 = 1; let mut played_1: Vec<ObjectId> = Vec::new();
let limit_1_0: i64 = i64::from({ let q: &dyn Query = g; 4i64 });
while left_1 > 0 {
let mut open_1: Vec<ObjectId> = Vec::new(); for &c in &pool_1 { let ways = g.cast_ways_during_resolution_by(c, caster_1); if ways.iter().any(|w| ((g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery)) && (i64::from(w.mana_value)) <= (limit_1_0))) { open_1.push(c); } }
let picked_1 = if open_1.is_empty() { None } else if open_1.len() > 1 || (true && (!false || pool_1.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_1, asker.source, Purpose::CastFree, pool_1.clone(), open_1.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_1, open_1[0], Purpose::CastFree)? { Some(open_1[0]) } else { None };
let Some(card_1) = picked_1 else { break; };
let ways_1: Vec<CastWay> = g.cast_ways_during_resolution_by(card_1, caster_1).into_iter().filter(|w| ((g.card(w.face).types.has(CardType::Instant) || g.card(w.face).types.has(CardType::Sorcery)) && (i64::from(w.mana_value)) <= (limit_1_0))).collect();
let done_1 = if ways_1.is_empty() { g.play_land_during_resolution(card_1)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_1, card_1, ways_1)?.map(|w| w.way); g.cast_during_resolution_as(card_1, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_1), exile_instead: ExileInstead::Spell, ..CastTerms::default() })? };
if done_1 { left_1 -= 1; played_1.push(g.state().obj(card_1).became.unwrap_or(card_1)); } else if true { break; } if done_1 || !(true && true) { pool_1.retain(|&c| c != card_1); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Diviner of Mist", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature attacks, mill four cards. You may cast an instant or sorcery spell from your graveyard with mana value 4 or less without paying its mana cost. If that spell would be put into your graveyard, exile it instead.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
