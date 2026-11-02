// Ponder
// status: ready
// Look at the top three cards of your library, then put them back in any order. You may shuffle.
// Draw a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let looked_0 = cantrip_mechanics::top_of_library(g, item.controller, 3); if !looked_0.is_empty() { let ask = RequestKind::DistributeCards { source: item.object, cards: looked_0.clone(), destinations: vec![Destination { kind: DestinationKind::LibraryTop, min: 0, max: u8::MAX, eligible: looked_0 }], may_shuffle: true }; if let Answer::Placements { placements, shuffle } = g.ask(item.controller, ask)? { if shuffle { g.shuffle(item.controller); } else { g.arrange_library(item.controller, &placements[0], &[]); } } }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ponder", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
