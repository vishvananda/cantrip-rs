// Dark Deal
// status: ready
// Each player discards all the cards in their hand, then draws that many cards minus one.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend(g.state().zone(Zone::hand(bound_0)).to_vec());
    } 
let cards_3 = { let q: &dyn Query = g; gathered_1.clone() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let amount_5 = { let q: &dyn Query = g; ((((discarded_3.clone()).into_iter().filter(|&o| Some(q.state().obj(o).owner) == Some(bound_0)).collect::<Vec<ObjectId>>()).len() as i64 - 1i64)).max(0) as u32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.draw(player, amount_5); }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Dark Deal", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
