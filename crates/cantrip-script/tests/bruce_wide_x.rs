//! Bruce Banner's two-X ability after a generic activation cost reduction.
use cantrip_engine::*;
use cantrip_state::{Frame, FrameKind};
const YOU: PlayerId = PlayerId(0);
fn reduction(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, _: &[Target]) -> CostDelta {
    if q.controller(source) == player && matches!(costed, Costed::Ability(..)) { CostDelta::less(2) } else { CostDelta::NONE }
}
static REDUCER: CardDef = CardDef { name: "Wide X reducer", oracle: true, types: TypeSet::of(&[CardType::Artifact]), cost_changes: &[reduction], ..CardDef::BLANK };
#[test]
fn bruce_two_x_reduction_is_applied_before_narrowing() {
    let table: &'static CardTable = Box::leak(Box::new(CardTable::new(cantrip_cards::all().into_iter().chain([&REDUCER]))));
    let all = cantrip_cards::all();
    let island = table.id_of(all.iter().copied().find(|c| c.name == "Island").unwrap());
    let bruce = table.id_of(all.iter().copied().find(|c| c.name == "Bruce Banner").unwrap());
    let mut g = Game::new(table, &Setup { decks: vec![vec![island; 20], vec![island; 20]], ..Default::default() });
    while !g.state.agenda.is_empty() { g.state.frame_remove(0); }
    g.state.update_turn(|t| { t.active = YOU; t.priority = YOU; t.set_step(Step::PrecombatMain); });
    g.state.frame_push(Frame::new(FrameKind::Priority));
    g.state.update_player(YOU, |p| p.turn_began = 1);
    let zone = Zone::BATTLEFIELD;
    let source = g.state.push_obj(Obj::new(bruce, YOU, zone));
    g.state.zone_push(zone, source);
    let reducer = g.state.push_obj(Obj::new(table.id_of(&REDUCER), YOU, zone));
    g.state.zone_push(zone, reducer);
    g.state.add_mana(YOU, Mana::C, i32::MAX as u32, ObjectId::NONE, None, ManaLasts::Step, false);
    g.state.add_mana(YOU, Mana::B, 1u32 << 31, ObjectId::NONE, None, ManaLasts::Step, false);
    g.run();
    let request = g.pending().cloned().unwrap();
    let RequestKind::Priority { options } = &request.kind else { panic!("priority") };
    let option = options.iter().find(|o| o.source == Some(source) && o.index == 0).unwrap();
    g.answer(request.id, YOU, Answer::Act(Action::of(option))).unwrap();
    g.run();
    let request = g.pending().cloned().unwrap();
    assert!(matches!(request.kind, RequestKind::PayCosts { announcing: true, .. }));
    g.answer(request.id, YOU, Answer::Announce(1u32 << 31)).unwrap();
    g.run();
    let request = g.pending().cloned().unwrap();
    let RequestKind::PayCosts { announcing: false, total, options, .. } = &request.kind else { panic!("payment") };
    assert_eq!(total.generic, u32::MAX - 1);
    let black = options.iter().find(|o| o.kind == ActionKind::AssignMana && o.mana == Some(Mana::B)).unwrap();
    g.answer(request.id, YOU, Answer::Act(Action::of(black))).unwrap();
    g.run();
    let request = g.pending().cloned().unwrap();
    assert!(matches!(request.kind, RequestKind::ChooseNumber { least: 1, most: 2147483648, heading: "Spend mana", .. }));
    g.answer(request.id, YOU, Answer::Announce(1u32 << 31)).unwrap();
    g.run();
    assert_eq!(g.state.player(YOU).pool.get(Mana::B), 0);
    assert!(g.pending().is_some(), "the next cost-order checkpoint remains available");
}

// Bane of the Living is not in the card table; this fixture uses its printed morph cost.
static BANE_MORPH: CardDef = CardDef {
    name: "Bane morph cost probe", oracle: true, types: TypeSet::of(&[CardType::Creature]), power: Some(2), toughness: Some(2),
    ways: &[cantrip_mechanics::morph::way(Cost::mana(mana("{X}{B}{B}")), "Morph {X}{B}{B}")],
    ..CardDef::BLANK
};
#[test]
fn bane_morph_rejects_an_unrepresentable_total_without_crashing() {
    let table: &'static CardTable = Box::leak(Box::new(CardTable::new(cantrip_cards::all().into_iter().chain([&BANE_MORPH]))));
    let all = cantrip_cards::all();
    let island = table.id_of(all.iter().copied().find(|c| c.name == "Island").unwrap());
    let bane = table.id_of(&BANE_MORPH);
    let mut g = Game::new(table, &Setup { decks: vec![vec![island; 20], vec![island; 20]], ..Default::default() });
    while !g.state.agenda.is_empty() { g.state.frame_remove(0); }
    g.state.update_turn(|t| { t.active = YOU; t.priority = YOU; t.set_step(Step::PrecombatMain); });
    g.state.frame_push(Frame::new(FrameKind::Priority));
    g.state.update_player(YOU, |p| p.turn_began = 1);
    let zone = Zone::BATTLEFIELD;
    let mut obj = Obj::new(bane, YOU, zone);
    obj.face_down = true;
    obj.marks.face_down_as = Some(table.face_down_values(FaceDown::Manifested));
    let source = g.state.push_obj(obj);
    g.state.zone_push(zone, source);
    g.state.add_mana(YOU, Mana::B, 3, ObjectId::NONE, None, ManaLasts::Step, false);
    g.run();
    let request = g.pending().cloned().unwrap();
    let RequestKind::Priority { options } = &request.kind else { panic!("priority") };
    let option = options.iter().find(|o| o.source == Some(source) && o.index == 1).expect("morph is offered");
    g.answer(request.id, YOU, Answer::Act(Action::of(option))).unwrap();
    g.run();
    let request = g.pending().cloned().unwrap();
    assert!(matches!(request.kind, RequestKind::PayCosts { announcing: true, .. }), "{:?}", request.kind);
    g.answer(request.id, YOU, Answer::Announce(u32::MAX - 1)).unwrap();
    g.run();
    assert!(g.state.obj(source).face_down, "unpayable morph is taken back");
    assert_eq!(g.state.player(YOU).pool.get(Mana::B), 3);

    // The same special action still accepts and pays a representable X.
    let request = g.pending().cloned().unwrap();
    let RequestKind::Priority { options } = &request.kind else { panic!("priority after rollback") };
    let option = options.iter().find(|o| o.source == Some(source) && o.index == 1).expect("morph remains offered");
    g.answer(request.id, YOU, Answer::Act(Action::of(option))).unwrap();
    g.run();
    let request = g.pending().cloned().unwrap();
    assert!(matches!(request.kind, RequestKind::PayCosts { announcing: true, .. }));
    g.answer(request.id, YOU, Answer::Announce(1)).unwrap();
    g.run();
    assert!(!g.state.obj(source).face_down, "representable morph succeeds: {:?}", g.pending().map(|r| &r.kind));
}
