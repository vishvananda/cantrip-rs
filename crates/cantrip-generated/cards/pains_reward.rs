// Pain's Reward
// status: planned
// requires: numeric expression Game
// requires: operation Bid
// requires: player reference Prior
// Each player may bid life. You start the bidding with a bid of any number. In turn order, each player may top the high bid. The bidding ends if the high bid stands. The high bidder loses life equal to the high bid and draws four cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-bid" => cantrip_mechanics::bid::bid(g, asker.source, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, None)?);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::WinningBid))).clamp(0, 2147483647) as i32 }; let players_1 = { let q: &dyn Query = g; planned_value!("player reference Prior", None::<PlayerId> => g.winning_bidder(item)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
let amount_2 = { let q: &dyn Query = g; (4i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; planned_value!("player reference Prior", None::<PlayerId> => g.winning_bidder(item)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Pain's Reward", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
