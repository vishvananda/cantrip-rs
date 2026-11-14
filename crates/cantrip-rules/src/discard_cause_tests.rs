//! Immediate discard attribution: Library of Leng and Psychic Purge use the same rules authority.
use crate::{Cx, Game, Setup};
use cantrip_api::*;
use cantrip_state::{Frame, FrameKind};

const A: PlayerId = PlayerId(0);
const B: PlayerId = PlayerId(1);
fn stack_discard(g: &mut dyn cantrip_api::Game, _: &StackItem) -> Result<(), Suspend> {
    let card = g.state().zone(Zone::hand(A)).first().copied();
    if let Some(card) = card { g.discard(card); }
    Ok(())
}
static SOURCE: CardDef = CardDef { name: "Discard cause source", types: TypeSet::of(&[CardType::Artifact]),
    abilities: &[ability(Cost::NONE, stack_discard, "Discard a card.")], ..CardDef::BLANK };
fn mana_discard(g: &mut dyn cantrip_api::Game, a: &Asker, _: Option<Color>) -> Result<(), Suspend> {
    if let Some(card) = g.state().zone(Zone::hand(B)).first().copied() { g.discard(card); }
    g.add_mana(a.controller, Mana::C, 1);
    Ok(())
}
static MANA_SOURCE: CardDef = CardDef { name: "Discard cause mana", types: TypeSet::of(&[CardType::Artifact]),
    abilities: &[mana_ability(Cost::NONE, add_by(mana_discard), "Discard a card. Add {C}.")], ..CardDef::BLANK };
fn caused_discard(_: &dyn Query, _: ObjectId, event: &Event) -> Option<Fired> {
    match *event {
        Event::Discarded { cause: DiscardCause::Effect { controller }, .. } => Some(Fired { subject: Target::None, data: [0, u32::from(controller.0)] }),
        _ => None,
    }
}
fn loss_of_cause_controller(g: &mut dyn cantrip_api::Game, item: &StackItem) -> Result<(), Suspend> {
    g.lose_life(PlayerId(item.data[1] as u8), 5);
    Ok(())
}
static WATCHER: CardDef = CardDef { name: "Discard cause watcher", types: TypeSet::of(&[CardType::Enchantment]),
    abilities: &[triggered(when(EventMask::DISCARDED, caused_discard), loss_of_cause_controller, "The effect controller loses 5 life.")], ..CardDef::BLANK };
fn entering_discard(g: &mut dyn cantrip_api::Game, _: &mut Entering) -> Result<(), Suspend> {
    if let Some(card) = g.state().zone(Zone::hand(B)).first().copied() { g.discard(card); }
    Ok(())
}
static ENTERS_DISCARD: CardDef = CardDef { name: "Discard cause entry replacement", types: TypeSet::of(&[CardType::Artifact]),
    enters: Some(entering_discard), ..CardDef::BLANK };
fn applies_b(_: &dyn Query, _: &Asker, p: &Proposal) -> bool {
    matches!(p, Proposal::ZoneChange(z) if z.reason == ZoneReason::Discard && z.discard_cause == Some(DiscardCause::Effect { controller: B }))
}
fn discard_a(g: &mut dyn cantrip_api::Game, _: &Asker, _: &Proposal, _: &Proposal) {
    if let Some(card) = g.state().zone(Zone::hand(A)).first().copied() { g.discard(card); }
}
static REVERSE: CardDef = CardDef { name: "Discard reverse controller replacement", types: TypeSet::of(&[CardType::Enchantment]),
    replacements: &[replacement(ReplacementKind::ZoneChange, applies_b, no_change, "Then discard another card.").then(discard_a)], ..CardDef::BLANK };
static CARD: CardDef = CardDef { name: "Discard cause card", types: TypeSet::of(&[CardType::Sorcery]), ..CardDef::BLANK };
fn applies(_: &dyn Query, _: &Asker, p: &Proposal) -> bool {
    matches!(p, Proposal::ZoneChange(z) if z.reason == ZoneReason::Discard && z.to.is(ZoneKind::Graveyard) && z.discard_cause == Some(DiscardCause::Effect { controller: A }))
}
fn redirect(_: &dyn Query, _: &Asker, p: &mut Proposal) {
    if let Proposal::ZoneChange(z) = p { z.to = Zone::library(A); }
}
fn another_discard(g: &mut dyn cantrip_api::Game, _: &Asker, _: &Proposal, _: &Proposal) {
    let card = g.state().zone(Zone::hand(B)).first().copied();
    if let Some(card) = card { g.discard(card); }
}
fn cost_applies(_: &dyn Query, _: &Asker, p: &Proposal) -> bool {
    matches!(p, Proposal::ZoneChange(z) if z.reason == ZoneReason::Discard && z.discard_cause == Some(DiscardCause::Cost))
}
fn no_change(_: &dyn Query, _: &Asker, _: &mut Proposal) {}
fn cancel_discard(_: &dyn Query, _: &Asker, p: &mut Proposal) {
    if let Proposal::ZoneChange(z) = p { z.replaced = true; }
}
static CANCEL_COST: CardDef = CardDef { name: "Cancel discard payment", types: TypeSet::of(&[CardType::Enchantment]),
    replacements: &[replacement(ReplacementKind::ZoneChange, cost_applies, cancel_discard, "The discard does nothing instead.")], ..CardDef::BLANK };
static COST_REPLACEMENT: CardDef = CardDef { name: "Discard cost replacement", types: TypeSet::of(&[CardType::Enchantment]),
    replacements: &[replacement(ReplacementKind::ZoneChange, cost_applies, no_change, "Then discard another card.").then(another_discard)], ..CardDef::BLANK };
const OPTIONAL: Question = Question::new("DiscardCauseRedirect", "Put it on top?", "", "Yes", "No");
static OPTIONAL_REDIRECT: CardDef = CardDef { name: "Discard cause optional replacement", types: TypeSet::of(&[CardType::Enchantment]),
    replacements: &[replacement(ReplacementKind::ZoneChange, applies, redirect, "You may put it on top instead.").optional(&OPTIONAL)], ..CardDef::BLANK };
static REDIRECT: CardDef = CardDef { name: "Discard cause replacement", types: TypeSet::of(&[CardType::Enchantment]),
    replacements: &[replacement(ReplacementKind::ZoneChange, applies, redirect, "Put it on top instead.").then(another_discard)], ..CardDef::BLANK };
fn game() -> Game {
    let table = Box::leak(Box::new(CardTable::new([&CARD, &REDIRECT, &SOURCE, &COST_REPLACEMENT, &MANA_SOURCE, &OPTIONAL_REDIRECT, &WATCHER, &ENTERS_DISCARD, &REVERSE])));
    let mut g = Game::new(table, &Setup { decks: vec![Vec::new(), Vec::new()], ..Default::default() });
    while !g.state.agenda.is_empty() { g.state.frame_remove(0); }
    for player in [A, B] { g.state.update_player(player, |p| p.drew_from_empty = false); }
    g.state.frame_push(Frame::new(FrameKind::Settle));
    g
}
fn put(g: &mut Game, def: &'static CardDef, owner: PlayerId, zone: Zone) -> ObjectId {
    let id = g.state.push_obj(Obj::new(g.cards.id_of(def), owner, zone));
    g.state.zone_push(zone, id);
    g.state.see_statics();
    id
}
fn cx(g: &mut Game) -> Cx<'_> {
    Cx { g, frame: 0, cursor: 0, mana_source: None, entering: None, lineage: Vec::new(), damage: None, asked_at: None, doer: None, discard_cause: None, producing: None }
}
fn cause(g: &Game, old: ObjectId) -> DiscardCause {
    let new = g.state.obj(old).became.expect("discarded");
    g.log.iter().find_map(|r| match r.event { Event::Discarded { card, cause, .. } if card == new => Some(cause), _ => None }).expect("discard event")
}
#[test]
fn mixed_cost_and_effect_discard_freezes_each_cause() {
    let mut g = game();
    let effect = put(&mut g, &CARD, A, Zone::hand(A));
    let cost = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| crate::ops::discard_paying(c, &[effect, cost], &[cost]));
    assert_eq!(cause(&g, effect), DiscardCause::Effect { controller: B });
    assert_eq!(cause(&g, cost), DiscardCause::Cost);
}
#[test]
fn replacement_new_discard_uses_its_controller_and_restores_outer_cause() {
    let mut g = game();
    put(&mut g, &REDIRECT, B, Zone::BATTLEFIELD);
    let outer = put(&mut g, &CARD, A, Zone::hand(A));
    let nested = put(&mut g, &CARD, B, Zone::hand(B));
    let after = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: A }, |c| {
        crate::ops::discard(c, outer);
        c.with_discard_cause(DiscardCause::Cost, |c| crate::ops::discard(c, after));
    });
    assert_eq!(cause(&g, outer), DiscardCause::Effect { controller: A });
    assert_eq!(cause(&g, nested), DiscardCause::Effect { controller: B });
    assert_eq!(cause(&g, after), DiscardCause::Cost);
    assert!(g.state.obj(g.state.obj(outer).became.unwrap()).zone.is(ZoneKind::Library));
    assert!(g.log.iter().any(|r| matches!(r.event, Event::Discarded { undefined: true, .. })));
}
#[test]
fn simultaneous_instruction_carries_effect_cause() {
    let mut g = game();
    let card = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| crate::ops::together(c, Together { discards: &[card], ..Together::default() }));
    assert_eq!(cause(&g, card), DiscardCause::Effect { controller: B });
}

#[test]
fn actual_stack_resolution_freezes_the_stack_controller() {
    let mut g = game();
    let source = put(&mut g, &SOURCE, A, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, A, Zone::hand(A));
    let item = crate::actions::stack_item(StackKind::Activated, source, AbilityRef { card: g.cards.id_of(&SOURCE), index: 0 }, B, &[]);
    g.state.stack_push(item);
    let serial = g.state.stack.last().unwrap().serial;
    g.state.frame_remove(0);
    g.state.frame_push(Frame::new(FrameKind::Resolve { serial }));
    crate::resolve::resolve(&mut cx(&mut g)).unwrap();
    g.state.update_obj(source, |o| o.controller = A);
    assert_eq!(cause(&g, card), DiscardCause::Effect { controller: B });
}

#[test]
fn mixed_together_distinguishes_cost_from_effect() {
    let mut g = game();
    let effect = put(&mut g, &CARD, A, Zone::hand(A));
    let cost = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| crate::ops::together(c, Together {
        discards: &[effect], discard_costs: &[cost], life: &[(A, -1)], ..Together::default()
    }));
    assert_eq!(cause(&g, effect), DiscardCause::Effect { controller: B });
    assert_eq!(cause(&g, cost), DiscardCause::Cost);
    assert_eq!(g.state.player(A).life, 19);
}

#[test]
fn replacement_inside_cost_is_its_own_effect_and_restores_cost() {
    let mut g = game();
    put(&mut g, &COST_REPLACEMENT, B, Zone::BATTLEFIELD);
    let first = put(&mut g, &CARD, A, Zone::hand(A));
    let nested = put(&mut g, &CARD, B, Zone::hand(B));
    let second = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Cost, |c| {
        crate::ops::discard(c, first);
        crate::ops::discard(c, second);
    });
    assert_eq!(cause(&g, first), DiscardCause::Cost);
    assert_eq!(cause(&g, nested), DiscardCause::Effect { controller: B });
    assert_eq!(cause(&g, second), DiscardCause::Cost);
}

#[test]
fn mana_ability_nested_in_another_effect_has_its_own_controller() {
    let mut g = game();
    let source = put(&mut g, &MANA_SOURCE, B, Zone::BATTLEFIELD);
    let nested = put(&mut g, &CARD, B, Zone::hand(B));
    let outer = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: A }, |c| {
        crate::actions::activate_mana(c, B, source, 0, None, false, &[]).unwrap();
        crate::ops::discard(c, outer);
    });
    assert_eq!(cause(&g, nested), DiscardCause::Effect { controller: B });
    assert_eq!(cause(&g, outer), DiscardCause::Effect { controller: A });
}

#[test]
fn cleanup_discard_is_a_turn_action() {
    let mut g = game();
    let mut cards = Vec::new();
    for _ in 0..8 { cards.push(put(&mut g, &CARD, A, Zone::hand(A))); }
    g.state.update_frame(0, |f| f.answers.push((None, Answer::Cards(vec![cards[0]]))));
    crate::turn::cleanup(&mut cx(&mut g)).unwrap();
    assert_eq!(cause(&g, cards[0]), DiscardCause::TurnOrRule);
}

#[test]
fn choice_suspend_clone_and_goto_keep_original_cause() {
    let mut g = game();
    put(&mut g, &OPTIONAL_REDIRECT, B, Zone::BATTLEFIELD);
    let source = put(&mut g, &SOURCE, B, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, A, Zone::hand(A));
    let item = crate::actions::stack_item(StackKind::Activated, source, AbilityRef { card: g.cards.id_of(&SOURCE), index: 0 }, A, &[]);
    g.state.stack_push(item);
    let serial = g.state.stack.last().unwrap().serial;
    g.state.frame_remove(0);
    g.state.frame_push(Frame::new(FrameKind::Resolve { serial }));
    g.run();
    let request = g.pending().cloned().expect("optional replacement choice");
    assert!(matches!(request.kind, RequestKind::May { .. }));
    let before = g.at;
    let mut declined = g.clone();
    declined.answer(request.id, request.player, Answer::No).unwrap();
    declined.run();
    assert_eq!(cause(&declined, card), DiscardCause::Effect { controller: A });
    assert!(declined.state.obj(declined.state.obj(card).became.unwrap()).zone.is(ZoneKind::Graveyard));
    g.answer(request.id, request.player, Answer::Yes).unwrap();
    g.run();
    let after = g.at;
    assert_eq!(cause(&g, card), DiscardCause::Effect { controller: A });
    assert!(g.state.obj(g.state.obj(card).became.unwrap()).zone.is(ZoneKind::Library));
    g.goto(before);
    assert_eq!(g.state.obj(card).zone, Zone::hand(A));
    g.goto(after);
    assert_eq!(cause(&g, card), DiscardCause::Effect { controller: A });
}

#[test]
fn discard_trigger_carries_the_frozen_controller_until_resolution() {
    let mut g = game();
    put(&mut g, &WATCHER, A, Zone::BATTLEFIELD);
    let source = put(&mut g, &SOURCE, B, Zone::BATTLEFIELD);
    let paid = put(&mut g, &CARD, A, Zone::hand(A));
    let effected = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Cost, |c| crate::ops::discard(c, paid));
    assert!(c.s().triggers.is_empty());
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| crate::ops::discard(c, effected));
    assert_eq!(c.s().triggers.len(), 1);
    assert_eq!(c.s().triggers[0].data[1], u32::from(B.0));
    c.st().update_obj(source, |o| o.controller = A);
    crate::settle::settle(&mut c).unwrap();
    let item = *c.s().stack.last().expect("discard trigger on stack");
    assert_eq!(item.data[1], u32::from(B.0));
    let frame = c.frame;
    c.st().update_frame(frame, |f| f.kind = FrameKind::Resolve { serial: item.serial });
    crate::resolve::resolve(&mut c).unwrap();
    assert_eq!(c.s().player(B).life, 15);
    assert_eq!(c.s().player(A).life, 20);
}

fn printed_game(name: &str) -> (Game, &'static CardDef) {
    let printed = cantrip_cards::all().into_iter().find(|d| d.name == name).expect("printed card");
    let table = Box::leak(Box::new(CardTable::new([&CARD, &CANCEL_COST, printed])));
    let mut g = Game::new(table, &Setup { decks: vec![vec![], vec![]], ..Default::default() });
    while !g.state.agenda.is_empty() { g.state.frame_remove(0); }
    for player in [A, B] { g.state.update_player(player, |p| p.drew_from_empty = false); }
    (g, printed)
}
fn run_printed(g: &mut Game, source: ObjectId, index: u8, kind: StackKind, subject: Target, data: [u32; 2], answers: Vec<Answer>) {
    let mut item = crate::actions::stack_item(kind, source, AbilityRef { card: g.state.obj(source).card, index }, A, &[]);
    item.subject = subject;
    item.data = data;
    g.state.stack_push(item);
    let serial = g.state.stack.last().unwrap().serial;
    g.state.frame_push(Frame::new(FrameKind::Resolve { serial }));
    g.state.update_frame(0, |f| f.answers = answers.into_iter().map(|a| (None, a)).collect());
    crate::resolve::resolve(&mut cx(g)).unwrap();
}
#[test]
fn printed_bebop_pays_a_resolving_discard_cost() {
    let (mut g, def) = printed_game("Bebop & Rocksteady");
    let source = put(&mut g, def, A, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, A, Zone::hand(A));
    run_printed(&mut g, source, 0, StackKind::Triggered, Target::None, [0; 2], vec![Answer::Act(Action::of(&ActionOption::simple(ActionKind::PayCost))), Answer::Cards(vec![card])]);
    assert!(g.state.obj(card).became.is_some(), "hand={:?}, stack={:?}, log={:?}", g.state.zone(Zone::hand(A)), g.state.stack, g.log);
    assert_eq!(cause(&g, card), DiscardCause::Cost);
}
#[test]
fn printed_ob_nixilis_discards_as_a_simultaneous_payment() {
    let (mut g, def) = printed_game("Ob Nixilis, the Adversary");
    let source = put(&mut g, def, A, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, B, Zone::hand(B));
    run_printed(&mut g, source, 1, StackKind::Activated, Target::None, [0; 2], vec![Answer::Yes, Answer::Cards(vec![card])]);
    assert_eq!(cause(&g, card), DiscardCause::Cost);
    assert_eq!(g.state.player(B).life, 20);
}
#[test]
fn printed_rottenmouth_discards_as_a_simultaneous_payment() {
    let (mut g, def) = printed_game("Rottenmouth Viper");
    let source = put(&mut g, def, A, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, B, Zone::hand(B));
    run_printed(&mut g, source, 0, StackKind::Triggered, Target::None, [0; 2], vec![Answer::Modes(vec![0]), Answer::Cards(vec![card])]);
    assert_eq!(cause(&g, card), DiscardCause::Cost);
    assert_eq!(g.state.player(B).life, 20);
}
#[test]
fn printed_wards_discard_as_chosen_and_random_costs() {
    for (name, random) in [("Sunset Saboteur", false), ("Alpharael, Stonechosen", true)] {
        let (mut g, def) = printed_game(name);
        let source = put(&mut g, def, A, Zone::BATTLEFIELD);
        let spell = put(&mut g, &CARD, B, Zone::STACK);
        let card = put(&mut g, &CARD, B, Zone::hand(B));
        if random { put(&mut g, &CARD, B, Zone::hand(B)); }
        let mut answers = vec![Answer::Yes];
        if !random { answers.push(Answer::Cards(vec![card])); }
        run_printed(&mut g, source, 0, StackKind::Triggered, Target::Object(spell), [u32::from(B.0), 0], answers);
        let discards: Vec<_> = g.log.iter().filter_map(|r| match r.event { Event::Discarded { cause, .. } => Some(cause), _ => None }).collect();
        assert_eq!(discards, [DiscardCause::Cost], "{name}");
        assert_eq!(g.state.zone(Zone::hand(B)).len(), usize::from(random), "{name}");
    }
}
#[test]
fn opponent_effect_restriction_still_applies_to_discard_payment() {
    let (mut g, def) = printed_game("Tamiyo, Collector of Tales");
    put(&mut g, def, B, Zone::BATTLEFIELD);
    let protected = put(&mut g, &CARD, B, Zone::hand(B));
    let allowed = put(&mut g, &CARD, A, Zone::hand(A));
    g.state.frame_push(Frame::new(FrameKind::Settle));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: A }, |c| {
        assert!(!cantrip_api::Game::may_pay_discard_cost(c, B));
        assert!(cantrip_api::Game::may_pay_discard_cost(c, A));
        assert!(cantrip_api::Game::discard_cost(c, &[protected]).is_empty());
        assert!(crate::ops::discard_paying(c, &[protected], &[protected]).is_empty());
        crate::ops::together(c, Together { discard_costs: &[protected], life: &[(A, -1)], ..Together::default() });
        assert_eq!(c.s().obj(protected).zone, Zone::hand(B));
        assert_eq!(cantrip_api::Game::discard_cost(c, &[allowed]).len(), 1);
    });
    assert_eq!(cause(&g, allowed), DiscardCause::Cost);
    assert_eq!(g.state.player(A).life, 19);
}

#[test]
fn fable_counts_only_cards_actually_discarded_when_a_cost_is_replaced() {
    let (mut g, def) = printed_game("Fable of the Mirror-Breaker");
    let source = put(&mut g, def, A, Zone::BATTLEFIELD);
    put(&mut g, &CANCEL_COST, A, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, A, Zone::hand(A));
    let top = put(&mut g, &CARD, A, Zone::library(A));
    run_printed(&mut g, source, 1, StackKind::Triggered, Target::None, [0; 2], vec![Answer::Cards(vec![card])]);
    assert_eq!(g.state.obj(card).zone, Zone::hand(A));
    assert_eq!(g.state.obj(top).zone, Zone::library(A));
}
#[test]
fn if_you_do_checks_started_payment_even_if_discard_is_replaced() {
    let (mut g, def) = printed_game("Bitter Reunion");
    let source = put(&mut g, def, A, Zone::BATTLEFIELD);
    put(&mut g, &CANCEL_COST, A, Zone::BATTLEFIELD);
    let card = put(&mut g, &CARD, A, Zone::hand(A));
    put(&mut g, &CARD, A, Zone::library(A));
    put(&mut g, &CARD, A, Zone::library(A));
    run_printed(&mut g, source, 0, StackKind::Triggered, Target::None, [0; 2], vec![Answer::Cards(vec![card])]);
    assert_eq!(g.state.obj(card).zone, Zone::hand(A));
    assert_eq!(g.state.zone(Zone::hand(A)).len(), 3);
    assert!(!g.log.iter().any(|r| matches!(r.event, Event::Discarded { .. })));
}

#[test]
fn entering_replacement_uses_the_entering_controllers_effect_scope() {
    let mut g = game();
    let entering = put(&mut g, &ENTERS_DISCARD, A, Zone::hand(A));
    let discarded = put(&mut g, &CARD, B, Zone::hand(B));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| {
        crate::ops::put_onto_battlefield(c, &[(entering, A, false)], ZoneReason::Other).unwrap();
    });
    assert_eq!(cause(&g, discarded), DiscardCause::Effect { controller: A });
}
#[test]
fn nested_mana_and_replacement_scopes_work_with_controllers_reversed() {
    let mut g = game();
    put(&mut g, &REVERSE, A, Zone::BATTLEFIELD);
    let nested = put(&mut g, &CARD, A, Zone::hand(A));
    let outer = put(&mut g, &CARD, B, Zone::hand(B));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| crate::ops::discard(c, outer));
    assert_eq!(cause(&g, nested), DiscardCause::Effect { controller: A });
    assert_eq!(cause(&g, outer), DiscardCause::Effect { controller: B });

    let mut g = game();
    let mana = put(&mut g, &MANA_SOURCE, A, Zone::BATTLEFIELD);
    let nested = put(&mut g, &CARD, B, Zone::hand(B));
    let outer = put(&mut g, &CARD, A, Zone::hand(A));
    let mut c = cx(&mut g);
    c.with_discard_cause(DiscardCause::Effect { controller: B }, |c| {
        crate::actions::activate_mana(c, A, mana, 0, None, false, &[]).unwrap();
        crate::ops::discard(c, outer);
    });
    assert_eq!(cause(&g, nested), DiscardCause::Effect { controller: A });
    assert_eq!(cause(&g, outer), DiscardCause::Effect { controller: B });
}

#[test]
fn competing_madness_and_library_redirect_preserve_the_original_cause() {
    let madness = cantrip_cards::all().into_iter().find(|d| d.name == "Fiery Temper").unwrap();
    let table = Box::leak(Box::new(CardTable::new([&CARD, &SOURCE, &OPTIONAL_REDIRECT, madness])));
    let mut g = Game::new(table, &Setup { decks: vec![vec![], vec![]], ..Default::default() });
    while !g.state.agenda.is_empty() { g.state.frame_remove(0); }
    for player in [A, B] { g.state.update_player(player, |p| p.drew_from_empty = false); }
    let redirect_source = put(&mut g, &OPTIONAL_REDIRECT, A, Zone::BATTLEFIELD);
    let source = put(&mut g, &SOURCE, B, Zone::BATTLEFIELD);
    let card = put(&mut g, madness, A, Zone::hand(A));
    let item = crate::actions::stack_item(StackKind::Activated, source, AbilityRef { card: g.cards.id_of(&SOURCE), index: 0 }, A, &[]);
    g.state.stack_push(item);
    let serial = g.state.stack.last().unwrap().serial;
    g.state.frame_push(Frame::new(FrameKind::Resolve { serial }));
    g.run();
    let request = g.pending().cloned().expect("order competing replacements");
    let RequestKind::ChooseReplacement { sources, .. } = &request.kind else { panic!("expected replacement order, got {:?}", request.kind) };
    let redirect_at = sources.iter().position(|&id| id == redirect_source).expect("redirect source");
    let madness_at = sources.iter().position(|&id| id == card).expect("madness source");
    let mut madness_first = g.clone();
    madness_first.answer(request.id, request.player, Answer::Index(madness_at as u8)).unwrap();
    madness_first.run();
    assert_eq!(cause(&madness_first, card), DiscardCause::Effect { controller: A });
    assert!(madness_first.state.obj(madness_first.state.obj(card).became.unwrap()).zone.is(ZoneKind::Exile));
    g.answer(request.id, request.player, Answer::Index(redirect_at as u8)).unwrap();
    g.run();
    let optional = g.pending().cloned().expect("optional redirect");
    assert!(matches!(optional.kind, RequestKind::May { .. }));
    g.answer(optional.id, optional.player, Answer::Yes).unwrap();
    g.run();
    assert_eq!(cause(&g, card), DiscardCause::Effect { controller: A });
    assert!(g.state.obj(g.state.obj(card).became.unwrap()).zone.is(ZoneKind::Library));
    assert!(g.log.iter().any(|r| matches!(r.event, Event::Discarded { undefined: true, .. })));
}
