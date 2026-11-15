#![cfg(feature = "cantrip")]
use mtgish_ast::{cantrip, ir::*, lower, mtg_types::{CardType, OracleCard}, validate};

fn card(name: &str) -> Card {
    let line = include_str!("fixtures/discard_causation.ronl").lines()
        .find(|line| line.contains(&format!("Name: {name:?}"))).unwrap();
    let source: OracleCard = ron::from_str(line).unwrap();
    let card = lower::card(&source).unwrap();
    validate::card(&card).unwrap();
    card
}
fn trigger(card: &mut Card) -> (&mut EventSpec, &mut Program) {
    let Ability::Scoped { ability, .. } = &mut card.abilities[1] else { panic!() };
    let Ability::Triggered { event, body, .. } = ability.as_mut() else { panic!() };
    (event, body)
}
fn with_cause(test: impl FnOnce()) {
    cantrip::planned::with_additional_features(&["discard-causation"], test);
}

#[test]
fn old_targets_keep_the_discard_authority_gaps() {
    cantrip::planned::with_additional_features(&[], || {
        assert!(cantrip::gaps(&card("Library of Leng")).iter()
            .any(|g| g.feature == "replacement event Discard"));
        assert!(cantrip::gaps(&card("Psychic Purge")).iter()
            .any(|g| g.feature == "engine: no discard cause in events"));
    });
}

#[test]
fn leng_tests_the_proposal_effect_class_without_an_ambient_or_source_lookup() {
    with_cause(|| {
        let card = card("Library of Leng");
        assert!(cantrip::gaps(&card).is_empty(), "{:?}", cantrip::gaps(&card));
        let code = cantrip::module(&card).unwrap();
        syn::parse_file(&code).unwrap();
        assert!(code.contains("matches!(z.discard_cause, Some(DiscardCause::Effect { .. }))"), "{code}");
        assert!(!code.contains("proposal_cause()"), "{code}");
        assert!(code.contains("put the card on top of its owner's library instead?"), "{code}");
        assert!(code.contains("Put on top"), "{code}");
        assert!(code.contains("Keep original destination"), "{code}");
        assert!(!code.contains("planned!"), "{code}");
    });
}

#[test]
fn purge_matches_and_carries_the_frozen_effect_controller() {
    with_cause(|| {
        let card = card("Psychic Purge");
        assert!(cantrip::gaps(&card).is_empty(), "{:?}", cantrip::gaps(&card));
        let code = cantrip::module(&card).unwrap();
        syn::parse_file(&code).unwrap();
        assert!(code.contains("matches!(*cause, DiscardCause::Effect { controller } if"), "{code}");
        assert!(code.contains("let data = [u32::from(player.0), match *cause"), "{code}");
        assert!(code.contains("u32::from(controller.0)"), "{code}");
        assert!(code.contains("data[1]"), "{code}");
        assert!(!code.contains("q.controller(cause)"), "{code}");
        assert!(!code.contains("planned!"), "{code}");
    });
}

#[test]
fn a_controller_only_authority_does_not_claim_causal_object_characteristics() {
    with_cause(|| {
        let mut card = card("Psychic Purge");
        let (EventSpec::Discard { kind, .. }, _) = trigger(&mut card) else { panic!() };
        *kind = DiscardKind::CausedBy(ObjectFilter::CardType(CardType::Sorcery));
        validate::card(&card).unwrap();
        assert!(cantrip::gaps(&card).iter().any(|g|
            g.feature == "discard cause object predicate CardType"));
    });
}

#[test]
fn grouped_cause_controller_cannot_overwrite_the_player_group_key() {
    with_cause(|| {
        let mut card = card("Psychic Purge");
        let (EventSpec::Discard { grouping, .. }, _) = trigger(&mut card) else { panic!() };
        *grouping = DiscardGrouping::Player;
        assert!(cantrip::gaps(&card).iter().any(|g|
            g.feature == "grouped discard cause controller carrier"));
    });
}

#[test]
fn undefined_characteristics_do_not_erase_an_unqualified_discard_event() {
    with_cause(|| {
        for characteristic_filter in [false, true] {
            let mut card = card("Psychic Purge");
            let (EventSpec::Discard { cards, kind, .. }, body) = trigger(&mut card) else { panic!() };
            *cards = if characteristic_filter { ObjectFilter::CardType(CardType::Creature) }
                else { ObjectFilter::Any };
            *kind = DiscardKind::Any;
            *body = Program { instructions: vec![Instruction::Effect {
                result: ResultId(0), operation: Operation::GainLife {
                    players: PlayerSet::One(PlayerRef::AbilityController), amount: Expr::Constant(1),
                },
            }] };
            validate::card(&card).unwrap();
            let code = cantrip::module(&card).unwrap();
            syn::parse_file(&code).unwrap();
            assert_eq!(code.contains("!*undefined"), characteristic_filter, "{code}");
        }
    });
}

#[test]
fn required_discard_cost_uses_cost_choices_and_moves_without_partial_payment() {
    // Native244 does not contain the pinned future API's World support. These
    // raw standalone modules exercise only the actual new discard interface.
    cantrip::planned::with_features(&["discard-causation", "32-bit-effect-quantities"], || {
        for random in [false, true] {
            for amount in [0, 2] {
                let mut card = card("Psychic Purge");
                card.abilities.truncate(1);
                let Ability::Spell { announcement, body } = &mut card.abilities[0] else { panic!() };
                announcement.targets.clear();
                *body = Program { instructions: vec![
                    Instruction::Effect { result: ResultId(0), operation: Operation::PayCost {
                        player: PlayerRef::AbilityController,
                        cost: Cost { parts: vec![CostPart::Discard {
                            count: Expr::Constant(amount), filter: ObjectFilter::Any, random,
                        }] }, optional: false,
                    } },
                    Instruction::If { condition: Condition::PaymentChosen(ResultId(0)),
                        then: Program { instructions: vec![Instruction::Effect {
                            result: ResultId(1), operation: Operation::GainLife {
                                players: PlayerSet::One(PlayerRef::AbilityController), amount: Expr::Constant(5),
                            },
                        }] }, otherwise: Program { instructions: vec![] },
                    },
                ] };
                validate::card(&card).unwrap();
                let code = cantrip::module(&card).unwrap();
                syn::parse_file(&code).unwrap();
                assert!(code.contains("g.discard_cost(&chosen_"), "{code}");
                assert!(code.contains("g.may_pay_discard_cost("), "{code}");
                assert!(code.contains(".len() == owed_"), "a cost needs the full count: {code}");
                assert!(code.contains("== 0 ||"), "zero is payable without discarding: {code}");
                assert_eq!(code.contains("choose::select_cost("), !random, "{code}");
                assert_eq!(code.contains("g.choose_at_random("), random, "{code}");
                assert!(!code.contains("discard_at_random(g"), "an effect verb cannot pay: {code}");
                // Standalone typed-IR API probes, not printed Psychic Purge
                // implementations. Native can include them without Oracle stubs.
                let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
                    .join("discard-cost-generated");
                std::fs::create_dir_all(&dir).unwrap();
                std::fs::write(dir.join(format!("required_discard_{random}_{amount}.rs")), code).unwrap();
            }
        }
    });
}

#[test]
fn printed_source_standalone_modules_use_the_actual_minimal_target_profile() {
    cantrip::planned::with_features(&["discard-causation", "32-bit-effect-quantities"], || {
        let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
            .join("discard-source-generated");
        std::fs::create_dir_all(&dir).unwrap();
        for (name, file) in [("Library of Leng", "library_of_leng.rs"),
                             ("Psychic Purge", "psychic_purge.rs")] {
            let card = card(name);
            let code = cantrip::module(&card).unwrap();
            syn::parse_file(&code).unwrap();
            assert!(!code.contains("world:"), "target has no World field: {code}");
            if name == "Psychic Purge" {
                assert!(code.contains("Vec<(ObjectId, Target, u32)>"), "{code}");
            }
            std::fs::write(dir.join(file), code).unwrap();
        }
    });
}
