// Dream Harvest
// status: ready
// Each opponent exiles cards from the top of their library until they have exiled cards with total mana value 5 or greater this way. Until end of turn, you may cast cards exiled this way without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let owner_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let mut total_1: i64 = 0; let mut objects_1: Vec<ObjectId> = Vec::new(); while total_1 < 5 { let Some(&top) = g.state().zone(Zone::library(owner_1)).first() else { break; }; let card = g.exile(top); if g.obj(card).zone.is(ZoneKind::Exile) { total_1 += i64::from(g.mana_value(card)); objects_1.push(card); } }
 each_2[index_0] = objects_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let moved_3: Vec<ObjectId> = { let q: &dyn Query = g; all_2.clone() };
g.end_sentence();
let cards_4_0 = { let q: &dyn Query = g; moved_3.clone() }; let players_4_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_4_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_4_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::EndOfTurn, instead: Some(ManaCost::ZERO), ..PlayGrant::CAST }); } } } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Dream Harvest", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::U, Mana::B)),Some((Mana::U, Mana::B)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
