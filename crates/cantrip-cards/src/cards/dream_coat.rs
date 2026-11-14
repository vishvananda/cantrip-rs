use cantrip_api::*;
use cantrip_mechanics::{choose_one_or_more_colors, targets};
use cantrip_oracle as oracle;

/// "{0}: Enchanted creature becomes the color or colors of your choice. Activate only once each turn." (CR 608.2, 611.2a).
fn enchanted_creature_becomes_color_colors(
    g: &mut dyn Game,
    item: &StackItem,
) -> Result<(), Suspend> {
    let colors_0 = choose_one_or_more_colors(g, item.controller, item.object, item.ability.card)?;
    let objects_1 = {
        let q: &dyn Query = g;
        let source = item.object;
        let host = if targets::permanent(q, source) {
            q.attached_to(source)
        } else {
            q.obj(source).attached_to.filter(|&host| targets::permanent(q, host))
        };
        host.into_iter().collect::<Vec<ObjectId>>()
    };
    let kind_1_0_0 = EffectKind::SetColors(colors_0);
    instruction(
        g,
        |g| {
            for &object in &objects_1 {
                g.add_effect(item.object, object, kind_1_0_0, Duration::Indefinite);
            }
        },
    );
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[
        ability(
                cost("{0}"),
                enchanted_creature_becomes_color_colors,
                "{0}: Enchanted creature becomes the color or colors of your choice. Activate only once each turn.",
            )
            .once_each_turn(),
    ],
    enchant: Some(targets::CREATURE),
    labels: &["White", "Blue", "Black", "Red", "Green"],
    ..oracle::DREAM_COAT
};
