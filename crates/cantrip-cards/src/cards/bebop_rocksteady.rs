use cantrip_api::*;
use cantrip_mechanics::{sacrifice_chosen, triggers::THIS_ATTACKS_OR_BLOCKS};
use cantrip_oracle as oracle;

const ASK: Question = Question::new(
    "DiscardOrSacrifice",
    "discard a card?",
    "If you don't, sacrifice a permanent",
    "Discard a card",
    "Sacrifice a permanent",
);

/// "Sacrifice a permanent unless you discard a card" (CR 118.12a): the controller chooses to discard a card of their hand,
/// which they can only do with a card in it, or else sacrifices a permanent of their choice (any permanent they control,
/// this creature included).
fn sacrifice_unless_discard(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let you = item.controller;
    if !g.may_pay(you, item.object, item.object, discard_cards(1), &ASK)? {
        sacrifice_chosen(g, you, item.object, 1, |_| true)?;
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(
        THIS_ATTACKS_OR_BLOCKS,
        sacrifice_unless_discard,
        "Whenever Bebop & Rocksteady attack or block, sacrifice a permanent unless you discard a card.",
    )],
    ..oracle::BEBOP_ROCKSTEADY
};
