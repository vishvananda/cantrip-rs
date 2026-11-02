// Stroke of Luck
// status: ready
// Look at the top four cards of your library. Choose one of them and put all cards with the same name as the chosen card from among them into your hand. You lose 1 life for each card put into your hand this way. Put the rest on the bottom of your library in a random order.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_1 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((4i64).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_1, &objects_1);
let chooser_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_2: Vec<ObjectId> = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let mut picked_2: Vec<ObjectId> = Vec::new();
let one_2 = cantrip_mechanics::choose::cards(g, chooser_2, asker.source, Purpose::PutIntoHand, pool_2.clone(), 1, false)?; if let Some(&one) = one_2.first() { let q: &dyn Query = g; picked_2 = pool_2.iter().copied().filter(|&c| cantrip_mechanics::same_name(q, c, one)).collect(); }
let objects_3 = { let q: &dyn Query = g; picked_2.clone() }; let moved_3 = g.return_to_hand(&objects_3);
let amount_4 = { let q: &dyn Query = g; ((moved_3.clone()).len() as i64).min(i32::MAX.into()) as i32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.lose_life(player, amount_4); }
let library_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let cards_5: Vec<ObjectId> = { let q: &dyn Query = g; (objects_1.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }.into_iter().filter(|&o| g.state().obj(o).became.is_none() && g.state().obj(o).zone == Zone::library(library_5)).collect(); let placed_5 = cards_5.clone();
g.bottom_in_random_order(library_5, &cards_5);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stroke of Luck", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
