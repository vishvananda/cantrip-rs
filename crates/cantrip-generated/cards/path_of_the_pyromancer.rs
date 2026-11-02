// Path of the Pyromancer
// status: planned
// requires: condition MostVotes
// requires: engine: no planar deck
// requires: keyword action ChaosEnsues
// requires: operation Vote
// Discard all the cards in your hand. Add {R} for each card discarded this way, then draw that many cards plus one.
// Will of the Planeswalkers — Starting with you, each player votes for planeswalk or chaos. If planeswalk gets more votes, planeswalk. If chaos gets more votes or the vote is tied, chaos ensues.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_0 = Vec::new(); for player in players_0 { cards_0.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_0 = g.discard_together(&cards_0);
g.end_sentence();
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; ((discarded_0.clone()).len() as i64) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::R, mana_2_0);}
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (((discarded_0.clone()).len() as i64 + 1i64)).min(u32::MAX.into()) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_4 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_4.iter().position(|&p| Some(p) == first_4) { players_4.rotate_left(at); }
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let voters_5: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
planned!("operation-vote" => cantrip_mechanics::vote::ballot(g, asker.source, &voters_5, cantrip_mechanics::vote::Ballot::Words(&["Planeswalk", "Chaos"]), false)?);
    } 
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition MostVotes", false => g.votes().most(option)) } {
planned!("engine-no-planar-deck" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.planeswalk(player); });
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; planned_value!("condition MostVotes", false => g.votes().most(option)) } {
planned!("keyword-action-chaos-ensues" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::planechase::chaos_ensues(g, player)?; });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Path of the Pyromancer", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
