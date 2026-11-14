use cantrip_api::*;
use cantrip_mechanics::replacements::{instead_of_drawing, you_would_draw_a_card};
use cantrip_mechanics::{choose, library, select, top_of_library};
use cantrip_oracle as oracle;

/// "The next time you would draw a card this turn, instead" applies even with an empty library (CR 121.6a, 614.1a).
fn applies(q: &dyn Query, asker: &Asker, _: ObjectId, _: u32, proposal: &Proposal) -> bool {
    you_would_draw_a_card(q, asker, proposal)
}

fn replace_draw(q: &dyn Query, asker: &Asker, _: ObjectId, _: u32, proposal: &mut Proposal) {
    instead_of_drawing(q, asker, proposal);
}

/// "Look at the top X cards of your library, put all but one of them on the bottom of your library in a random order, then draw a card" (CR 701.20e, 121.1, 614.5).
fn look_and_draw(g: &mut dyn Game, asker: &Asker, x: u32) -> Result<(), Suspend> {
    let looked = top_of_library(g, asker.controller, x as usize);
    g.look_at(asker.controller, &looked);
    let chosen = choose::select(
        g,
        asker.controller,
        asker.source,
        Purpose::Choose,
        looked.clone(),
        select::Selection::of(select::exactly(1), &[])
            .called("card to leave on top of your library"),
    )?;
    let rest: Vec<_> = looked
        .into_iter()
        .filter(|card| !chosen.contains(card))
        .collect();
    library::rest_to_bottom_in_random_order(g, asker.controller, &rest);
    g.draw(asker.controller, 1);
    Ok(())
}

fn then(g: &mut dyn Game, asker: &Asker, _: ObjectId, x: u32, _: &Proposal, _: &Proposal) {
    let _ = look_and_draw(g, asker, x);
}

/// "{X}, {T}: The next time you would draw a card this turn" captures this activation's X (CR 107.3a, 611.2d), lasting this turn (CR 611.2a).
fn activate(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    g.add_player_effect(
        item.controller,
        item.object,
        EffectKind::CapturedReplacementRule {
            card: item.ability.card,
            index: 0,
            value: item.x,
        },
        Duration::EndOfTurn,
    );
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[ability(cost("{X}").and(TAP), activate,
        "{X}, {T}: The next time you would draw a card this turn, instead look at the top X cards of your library, put all but one of them on the bottom of your library in a random order, then draw a card. X can't be 0.").x_cant_be_zero()],
    granted_replacements: &[captured_granted_replacement(ReplacementKind::Draw,
        &CapturedReplacementCallbacks { applies, apply: replace_draw, then: Some(then), damage: None },
        "look at the top X cards and draw one instead").once().show_captured_value().text(
        "The next time you would draw a card this turn, instead look at the top X cards of your library, put all but one of them on the bottom of your library in a random order, then draw a card.")],
    ..oracle::ALADDINS_LAMP
};
