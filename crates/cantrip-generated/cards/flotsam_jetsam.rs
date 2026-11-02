// Flotsam // Jetsam
// status: ready
// Mill three cards. Investigate. (Create a Clue token. It's an artifact with "{2}, Sacrifice this token: Draw a card.")
// Each opponent mills three cards, then you may cast a spell from each opponent's graveyard without paying its mana cost. If a spell cast this way would be put into a graveyard, exile it instead.
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Flotsam // Jetsam", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::G, Mana::U)),Some((Mana::U, Mana::B)),Some((Mana::U, Mana::B)),None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant,CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, split: Some((&left::CARD, &right::CARD)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::tokens::CLUE,&left::CARD,&right::CARD];
pub mod left {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; for _ in 0..times { cantrip_mechanics::tokens::investigate(g, player)?; }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Flotsam", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::G, Mana::U)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
pub mod right {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
for owner_1 in { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() } {
let caster_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_2: u32 = 1; let mut played_2: Vec<ObjectId> = Vec::new();
pool_2.retain(|&c| g.state().obj(c).owner == owner_1);
while left_2 > 0 {
let mut open_2: Vec<ObjectId> = Vec::new(); for &c in &pool_2 { let ways = g.cast_ways_during_resolution_by(c, caster_2); if ways.iter().any(|w| true) { open_2.push(c); } }
let picked_2 = if open_2.is_empty() { None } else if open_2.len() > 1 || (true && (!false || pool_2.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_2, asker.source, Purpose::CastFree, pool_2.clone(), open_2.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_2, open_2[0], Purpose::CastFree)? { Some(open_2[0]) } else { None };
let Some(card_2) = picked_2 else { break; };
let ways_2: Vec<CastWay> = g.cast_ways_during_resolution_by(card_2, caster_2).into_iter().filter(|w| true).collect();
let done_2 = if ways_2.is_empty() { g.play_land_during_resolution(card_2)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_2, card_2, ways_2)?.map(|w| w.way); g.cast_during_resolution_as(card_2, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_2), exile_instead: ExileInstead::Spell, ..CastTerms::default() })? };
if done_2 { left_2 -= 1; played_2.push(g.state().obj(card_2).became.unwrap_or(card_2)); } else if true { break; } if done_2 || !(true && true) { pool_2.retain(|&c| c != card_2); }
}
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Jetsam", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::B)),Some((Mana::U, Mana::B)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
