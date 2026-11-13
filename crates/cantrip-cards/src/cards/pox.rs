use cantrip_api::*;
use cantrip_mechanics::{choose, select};
use cantrip_oracle as oracle;

/// "A third of" `n`, rounded up (CR 107.1a).
fn third(n: usize) -> u32 {
    n.div_ceil(3) as u32
}

/// "Each player sacrifices a third of the permanents of `kind` they control of their choice, rounded up" (CR 608.2, 701.21a, 101.4):
/// every player chooses on the board as it stands, then all are sacrificed together.
fn each_player_sacrifices_third(g: &mut dyn Game, item: &StackItem, kind: CardType) -> Result<(), Suspend> {
    let mut picks: Vec<(PlayerId, ObjectId)> = Vec::new();
    for player in g.state().apnap().collect::<Vec<_>>() {
        let controlled: Vec<ObjectId> = g.permanents_of(player).into_iter().filter(|&id| g.chars(id).is(kind)).collect();
        let count = third(controlled.len());
        let eligible: Vec<ObjectId> = controlled.into_iter().filter(|&id| g.can_sacrifice(player, id)).collect();
        let taken = if eligible.len() <= count as usize {
            eligible
        } else {
            choose::select(g, player, item.object, Purpose::Sacrifice, eligible, select::Selection::of(select::Count::AsManyAsCan(count), &[]))?
        };
        picks.extend(taken.into_iter().map(|object| (player, object)));
    }
    g.sacrifice_each(&picks);
    Ok(())
}

/// "Each player loses a third of their life, then discards a third of the cards in their hand, then sacrifices a third of the creatures they control of their choice, then sacrifices a third of the lands they control of their choice. Round up each time." (CR 608.2, 701.21a, 701.9a).
fn player_loses_third_their_life(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let losses: Vec<(PlayerId, i32)> = g
        .state()
        .apnap()
        .map(|player| (player, third(g.state().player(player).life.max(0) as usize) as i32))
        .collect();
    g.lose_life_together(&losses);
    g.end_sentence();

    let mut discards: Vec<ObjectId> = Vec::new();
    for player in g.state().apnap().collect::<Vec<_>>() {
        // CR 111.6, 704.5d: a token or a card copy in hand is not a card, so it is neither counted nor offered.
        let hand: Vec<_> = g.state().zone(Zone::hand(player)).iter().copied().filter(|&id| !g.obj(id).token && !g.obj(id).card_copy).collect();
        let count = third(hand.len());
        discards.extend(choose::select(g, player, item.object, Purpose::Discard, hand, select::cards(select::exactly(count)))?);
    }
    g.discard_together(&discards);
    g.end_sentence();

    each_player_sacrifices_third(g, item, CardType::Creature)?;
    g.end_sentence();
    each_player_sacrifices_third(g, item, CardType::Land)
}

pub static CARD: CardDef = CardDef {
    spell: spell(&[], player_loses_third_their_life),
    ..oracle::POX
};
