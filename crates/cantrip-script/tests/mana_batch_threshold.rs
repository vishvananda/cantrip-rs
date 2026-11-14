//! Small mana payments keep their normal flow; larger ones retain unit-level interleaving.
use cantrip_engine::*;
use cantrip_state::{Frame, FrameKind};
const YOU: PlayerId = PlayerId(0);
fn nothing(_: &mut dyn cantrip_api::Game, _: &StackItem) -> Result<(), Suspend> { Ok(()) }
static SMALL: CardDef = CardDef {
    name: "Small mana batching control", oracle: true, types: TypeSet::of(&[CardType::Artifact]),
    abilities: &[ability(cost("{15}, {T}"), nothing, "{15}, {T}: Nothing.")], ..CardDef::BLANK
};
static LARGE: CardDef = CardDef {
    name: "Large mana batching control", oracle: true, types: TypeSet::of(&[CardType::Artifact]),
    abilities: &[ability(cost("{16}, {T}"), nothing, "{16}, {T}: Nothing.")], ..CardDef::BLANK
};
fn offered(def: &'static CardDef, amount: u32) -> Game {
    let table: &'static CardTable = Box::leak(Box::new(CardTable::new(cantrip_cards::all().into_iter().chain([&SMALL, &LARGE]))));
    let island = table.id_of(cantrip_cards::all().into_iter().find(|c| c.name == "Island").unwrap());
    let mut g = Game::new(table, &Setup { decks: vec![vec![island; 20], vec![island; 20]], ..Default::default() });
    while !g.state.agenda.is_empty() { g.state.frame_remove(0); }
    g.state.update_turn(|t| { t.active = YOU; t.priority = YOU; t.set_step(Step::PrecombatMain); });
    g.state.frame_push(Frame::new(FrameKind::Priority));
    g.state.update_player(YOU, |p| p.turn_began = 1);
    let source = g.state.push_obj(Obj::new(table.id_of(def), YOU, Zone::BATTLEFIELD));
    g.state.zone_push(Zone::BATTLEFIELD, source);
    g.state.add_mana(YOU, Mana::C, amount, ObjectId::NONE, None, ManaLasts::Step, false);
    g.run();
    let r = g.pending().cloned().unwrap();
    let RequestKind::Priority { options } = &r.kind else { panic!("priority") };
    let option = options.iter().find(|o| o.source == Some(source) && o.index == 0).unwrap();
    g.answer(r.id, YOU, Answer::Act(Action::of(option))).unwrap();
    g.run();
    g
}
#[test]
fn fifteen_uses_ordinary_unit_flow() {
    let g = offered(&SMALL, 15);
    assert!(!matches!(g.pending().map(|r| &r.kind), Some(RequestKind::ChooseNumber { heading: "Spend mana", .. })));
    assert_eq!(g.state.player(YOU).pool.get(Mana::C), 0);
    assert!(!g.state.stack.is_empty(), "the activation was paid and placed on the stack");
}
#[test]
fn sixteen_offers_batch_and_retains_one_unit_choice() {
    let mut g = offered(&LARGE, 16);
    let r = g.pending().cloned().unwrap();
    assert!(matches!(r.kind, RequestKind::ChooseNumber { least: 1, most: 16, heading: "Spend mana", .. }), "{:?}", r.kind);
    g.answer(r.id, YOU, Answer::Announce(1)).unwrap();
    g.run();
    assert_eq!(g.state.player(YOU).pool.get(Mana::C), 0);
    assert!(!g.state.stack.is_empty(), "the activation completes after choosing one");
}
