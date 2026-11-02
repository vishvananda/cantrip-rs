// Khorvath's Fury
// status: ready
// For each player, choose friend or foe. Each friend discards all cards from their hand, then draws that many cards plus one. Khorvath's Fury deals damage to each foe equal to the number of cards in their hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let card_0 = g.card_id(&CARD); let mut words_0: Vec<(PlayerId, u8)> = Vec::new(); for player in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } { let word = cantrip_mechanics::choose::option(g, asker.controller, asker.source, card_0, 0..2)?; words_0.push((player, word)); }
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 0)).collect::<Vec<PlayerId>>() };
let mut gathered_2: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { gathered_2.extend(g.state().zone(Zone::hand(bound_0)).to_vec());
    } 
let cards_4 = { let q: &dyn Query = g; gathered_2.clone() }; let discarded_4 = g.discard_together(&cards_4);
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 0)).collect::<Vec<PlayerId>>() };
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { let amount_6 = { let q: &dyn Query = g; ((((discarded_4.clone()).into_iter().filter(|&o| Some(q.state().obj(o).owner) == Some(bound_0)).collect::<Vec<ObjectId>>()).len() as i64 + 1i64)).min(u32::MAX.into()) as u32 }; let players_6 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
    } 
g.end_sentence();
let mut players_7: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 1)).collect::<Vec<PlayerId>>() };
let mut gathered_8: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_7, bound_0) in players_7.iter().copied().enumerate() { let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let amount_9 = { let q: &dyn Query = g; ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).min(u16::MAX.into()) as u16 }; if let Some(source) = source_9 { for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { gathered_8.push((source, t, amount_9)); } }
    } 
g.damage_together(&gathered_8);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Khorvath's Fury", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Friend", "Foe"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
