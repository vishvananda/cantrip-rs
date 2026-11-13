use cantrip_api::*;
use cantrip_oracle as oracle;
use cantrip_mechanics::{choose_color, triggers};

/// "As this enchantment enters, choose a color. At the beginning of each opponent's upkeep, this enchantment deals X damage to that player, where X is the number of nontoken permanents of the chosen color they control. At the beginning of your upkeep, destroy this enchantment unless you sacrifice two Islands." (CR 614.1c).
fn as_enchantment_enters_choose_color_enters(
    g: &mut dyn Game,
    entering: &mut Entering,
) -> Result<(), Suspend> {
    entering.chosen_color = Some(choose_color(g, entering.controller, entering.object)?);
    Ok(())
}

/// "At the beginning of each opponent's upkeep, this enchantment deals X damage to that player, where X is the number of nontoken permanents of the chosen color they control." (CR 608.2, 120.3).
fn enchantment_deals_x_damage_player(
    g: &mut dyn Game,
    item: &StackItem,
) -> Result<(), Suspend> {
    // The upkeep is the active player's: that is the opponent the trigger is about.
    let player = g.state().turn.active;
    let chosen = g.state().chosen_color(g.linked(item));
    let x = g
        .permanents_of(player)
        .into_iter()
        .filter(|&id| {
            !g.obj(id).token && chosen.is_some_and(|color| g.chars(id).colors.has(color))
        })
        .count() as u32;
    g.damage(item.object, Target::Player(player), x);
    Ok(())
}

/// "Sacrifice two Islands", read through this enchantment's words (CR 612).
fn island(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool {
    q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Island))
}

/// "At the beginning of your upkeep, destroy this enchantment unless you sacrifice two Islands." (CR 608.2, 701.8a, 118.12): a payer
/// who can't sacrifice two Islands is not asked (CR 118.3).
fn destroy_enchantment_unless_sacrifice_two(
    g: &mut dyn Game,
    item: &StackItem,
) -> Result<(), Suspend> {
    let land = g.words(item.object).subtype(Subtype::Island);
    if !g.may_pay(item.controller, item.object, item.object, sacrifice(2, island), pay_question(land))? {
        g.destroy(item.object);
    }
    Ok(())
}

fn pay_question(land: Subtype) -> &'static Question {
    macro_rules! question {
        ($name:ident, $ask:literal, $yes:literal) => {
            static $name: Question = Question::new("Pay", $ask, "", $yes, "Destroy {subject}").no_default();
        };
    }
    question!(PLAINS, "sacrifice two Plains, or destroy {subject}?", "Sacrifice two Plains");
    question!(ISLANDS, "sacrifice two Islands, or destroy {subject}?", "Sacrifice two Islands");
    question!(SWAMPS, "sacrifice two Swamps, or destroy {subject}?", "Sacrifice two Swamps");
    question!(MOUNTAINS, "sacrifice two Mountains, or destroy {subject}?", "Sacrifice two Mountains");
    question!(FORESTS, "sacrifice two Forests, or destroy {subject}?", "Sacrifice two Forests");
    match land {
        Subtype::Plains => &PLAINS,
        Subtype::Swamp => &SWAMPS,
        Subtype::Mountain => &MOUNTAINS,
        Subtype::Forest => &FORESTS,
        _ => &ISLANDS,
    }
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        triggered(
            triggers::EACH_OPPONENTS_UPKEEP,
            enchantment_deals_x_damage_player,
            "At the beginning of each opponent's upkeep, this enchantment deals X damage to that player, where X is the number of nontoken permanents of the chosen color they control.",
        ),
        triggered(
            triggers::YOUR_UPKEEP,
            destroy_enchantment_unless_sacrifice_two,
            "At the beginning of your upkeep, destroy this enchantment unless you sacrifice two Islands.",
        ),
    ],
    enters: Some(as_enchantment_enters_choose_color_enters),
    ..oracle::PSYCHIC_ALLERGY
};
