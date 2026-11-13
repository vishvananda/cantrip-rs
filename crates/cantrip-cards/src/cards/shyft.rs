use cantrip_api::*;
use cantrip_mechanics::{choose_one_or_more_colors, effects, targets, triggers};
use cantrip_oracle as oracle;

/// Choose a nonempty set of colors, which lasts indefinitely (CR 105.2, 611.2a).
fn choose_colors(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    if effects::ask_may(g, item.controller, item.object, Purpose::Optional)? {
        let colors = choose_one_or_more_colors(g, item.controller, item.object, item.ability.card)?;
        if targets::permanent(g, item.object) {
            g.add_effect(item.object, item.object, EffectKind::SetColors(colors), Duration::Indefinite);
        }
    }
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[triggered(triggers::YOUR_UPKEEP, choose_colors,
        "At the beginning of your upkeep, you may have this creature become the color or colors of your choice.")],
    labels: &["White", "Blue", "Black", "Red", "Green"],
    ..oracle::SHYFT
};
