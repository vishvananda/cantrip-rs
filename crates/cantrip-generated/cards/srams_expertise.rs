// Sram's Expertise
// status: ready
// Create three 1/1 colorless Servo artifact creature tokens.
// You may cast a spell with mana value 3 or less from your hand without paying its mana cost.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Servo).name, token_words: token!(Servo).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Servo]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
let limit_2_0: i64 = i64::from({ let q: &dyn Query = g; 3i64 });
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| (i64::from(w.mana_value)) <= (limit_2_0)) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (true && (!false || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::CastFree, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::CastFree)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| (i64::from(w.mana_value)) <= (limit_2_0)).collect();
let done_2 = if ways_2.is_empty() { g.play_land_during_resolution(card_2)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_2), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else if true { break; } if done_2 || !(true && true) { pool_2.retain(|&c| c != card_2); }
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sram's Expertise", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
