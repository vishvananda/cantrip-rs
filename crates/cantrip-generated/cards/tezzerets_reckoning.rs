// Tezzeret's Reckoning
// status: planned
// requires: play permission for a number of cards
// Exile three random cards from your library face down and look at them. For as long as they remain exiled, you may play one of those cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_0 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; (3i64).max(0) as usize }; let mut picked_0: Vec<ObjectId> = Vec::new(); while picked_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); picked_0.push(pool_0.remove(at)); }
let objects_1 = { let q: &dyn Query = g; picked_0.clone() }; let by_1 = None; let moved_1 = g.exile_face_down(&objects_1, by_1);
g.end_sentence();
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_2 = { let q: &dyn Query = g; moved_1.clone() }; g.look_at(player_2, &objects_2);
g.end_sentence();
planned!("play-permission-for-a-number-of-cards" => let cards = { let q: &dyn Query = g; (moved_1.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>() }; let players = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for &player in &players { g.allow_play_some(&cards, player, PlayGrant { lands: true, expiry: Expiry::Never, ..PlayGrant::CAST }, 1); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Tezzeret's Reckoning", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
