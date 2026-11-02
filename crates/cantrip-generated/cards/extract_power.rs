// Extract Power
// status: ready
// Look at the top card of each player's library, then exile those cards face down. You may play them without paying their mana costs for as long as they remain exiled.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_0 = { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; g.look_at(player_0, &objects_0);
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().filter_map(|p: PlayerId| q.state().zone(Zone::library(p)).first().copied()).collect::<Vec<ObjectId>>() }; let by_1 = None; let moved_1 = g.exile_face_down(&objects_1, by_1);
g.end_sentence();
let cards_2_0 = { let q: &dyn Query = g; moved_1.clone() }; let players_2_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for &player in &players_2_0 { g.look_at(player, &cards_2_0); }
let cards_2_1 = { let q: &dyn Query = g; moved_1.clone() }; let players_2_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_2_1 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_2_1 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::Never, instead: Some(ManaCost::ZERO), ..PlayGrant::CAST }); } } } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Extract Power", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
