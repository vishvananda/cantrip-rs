//! A printed X spell beyond the old 1000 announcement cap (CR 601.2b).
use std::sync::OnceLock;
use cantrip_engine::*;
use cantrip_script::{GameScript, run_on};
fn enough_mana(g: &mut dyn cantrip_api::Game, item: &StackItem) -> Result<(), Suspend> {
    g.add_mana(item.controller, Mana::C, 1001);
    g.end_sentence();
    g.add_mana(item.controller, Mana::R, 1);
    Ok(())
}
static MANA: CardDef = CardDef {
    name: "X mana probe", oracle: true, has_cost: true, cost: ManaCost::ZERO,
    types: TypeSet::of(&[CardType::Sorcery]), spell: spell(&[], enough_mana), ..CardDef::BLANK
};
#[test]
fn fireball_uses_x_1001_with_sufficient_mana() {
    static TABLE: OnceLock<CardTable> = OnceLock::new();
    let table = TABLE.get_or_init(|| CardTable::new(cantrip_cards::all().into_iter().chain([&MANA])));
    let script = GameScript::parse(r#"
[Life]
Bot2: 2000
[Bot1 Deck]
X mana probe
Fireball
Mountain x20
[Bot2 Deck]
Island x20
[Actions]
Bot1.Hand.X mana probe.Cast
Bot1.Pass
Bot2.Pass
Bot1.Hand.Fireball.Cast(X: 1001)
Bot1.Target(Bot2)
Bot1.Pass
Bot2.Pass
[Expect]
Bot2.Life = 999
"#, "fireball-x-1001").unwrap();
    let result = run_on(&script, table).unwrap_or_else(|f| panic!("{}", f.report));
    let mut game = result.game;
    let end = game.at;
    let root = game.tree.root();
    game.goto(root);
    game.goto(end);
    assert_eq!(game.state.player(PlayerId(1)).life, 999);
    assert!(game.log.iter().any(|r| matches!(r.event, Event::Damage { amount: 1001, .. })));
}

fn reduced_mana(g: &mut dyn cantrip_api::Game, item: &StackItem) -> Result<(), Suspend> {
    g.add_mana(item.controller, Mana::C, 1002);
    g.end_sentence();
    g.add_mana(item.controller, Mana::R, 3);
    Ok(())
}
static REDUCED_MANA: CardDef = CardDef {
    name: "Reduced X mana probe", oracle: true, has_cost: true, cost: ManaCost::ZERO,
    types: TypeSet::of(&[CardType::Sorcery]), spell: spell(&[], reduced_mana), ..CardDef::BLANK
};
#[test]
fn fireball_x_1001_pays_its_reduced_actual_cost() {
    static TABLE: OnceLock<CardTable> = OnceLock::new();
    let table = TABLE.get_or_init(|| CardTable::new(cantrip_cards::all().into_iter().chain([&REDUCED_MANA])));
    let script = GameScript::parse(r#"
[Life]
Bot2: 2000
[Bot1 Deck]
Reduced X mana probe
The Fire Crystal
Fireball
Mountain x20
[Bot2 Deck]
Island x20
[Actions]
Bot1.Hand.Reduced X mana probe.Cast
Bot1.Pass
Bot2.Pass
Bot1.Hand.The Fire Crystal.Cast
Bot1.Pass
Bot2.Pass
Bot1.Hand.Fireball.Cast(X: 1001)
Bot1.Target(Bot2)
Bot1.Pass
Bot2.Pass
[Expect]
Bot2.Life = 999
"#, "fireball-reduced-x-1001").unwrap();
    let result = run_on(&script, table);
    assert!(result.is_ok(), "{}", result.err().map(|f| f.report).unwrap_or_default());
}

static BATTERY: CardDef = CardDef {
    name: "X mana battery probe", oracle: true, has_cost: true, cost: ManaCost::ZERO,
    types: TypeSet::of(&[CardType::Artifact]),
    abilities: &[mana_ability(TAP, Adds { pool: ManaPool([0, 0, 0, 0, 0, 1001]), ..add_n(Mana::C, 1) }, "{T}: Add 1001 colorless mana.")],
    ..CardDef::BLANK
};
#[test]
fn mana_abilities_remain_available_after_announcing_x_1001() {
    static TABLE: OnceLock<CardTable> = OnceLock::new();
    let table = TABLE.get_or_init(|| CardTable::new(cantrip_cards::all().into_iter().chain([&BATTERY])));
    let script = GameScript::parse(r#"
[Life]
Bot2: 2000
[Bot1 Deck]
X mana battery probe
Mox Ruby
Fireball
Mountain x20
[Bot2 Deck]
Island x20
[Actions]
Bot1.Hand.X mana battery probe.Cast
Bot1.Hand.Mox Ruby.Cast
Bot1.Hand.Fireball.Cast(X: 1001)
Bot1.Target(Bot2)
Bot1.Battlefield.X mana battery probe.Activate
Bot1.Battlefield.Mox Ruby.Activate
Bot1.Pass
Bot2.Pass
[Expect]
Bot2.Life = 999
"#, "fireball-x-1001-mana-window").unwrap();
    let result = run_on(&script, table);
    assert!(result.is_ok(), "{}", result.err().map(|f| f.report).unwrap_or_default());
}
