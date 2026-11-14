use cantrip_api::*;
use cantrip_mechanics::targets;
use cantrip_oracle as oracle;

/// Set this creature's base toughness to one plus that creature's power, indefinitely (CR 608.2h, 613.4b).
fn set_toughness(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> {
    let Some(creature) = targets::object(g, item, 0) else { return Ok(()) };
    let power = g.chars(creature).power.unwrap_or_default();
    g.add_effect(item.object, item.object, EffectKind::SetToughness(power + 1), Duration::Indefinite);
    Ok(())
}

pub static CARD: CardDef = CardDef {
    abilities: &[ability(cost("{0}"), set_toughness,
        "{0}: This creature's base toughness becomes equal to 1 plus the power of target creature blocking or blocked by this creature.")
        .targets(&[targets::CREATURE_BLOCKING_OR_BLOCKED_BY_SOURCE])],
    ..oracle::SENTINEL
};
