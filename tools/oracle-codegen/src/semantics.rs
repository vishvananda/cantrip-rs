//! IR-only activation classification retained from the extraction baseline.
use crate::ir::*;
use serde::Serialize;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ActivationKind {
    Mana,
    Stack,
}
#[derive(Default)]
struct Effects {
    prepared: std::collections::BTreeMap<u32, bool>,
    mana: bool,
    library: bool,
    objects: std::collections::BTreeMap<u32, (bool, bool)>,
    targets: std::collections::BTreeMap<u16, bool>,
}
impl Effects {
    fn program(&mut self, program: &Program) {
        for instruction in &program.instructions {
            match instruction {
                Instruction::Effect { result, operation } => match operation {
                    Operation::InOrder(actions) => {
                        for operation in actions {
                            self.program(&Program {
                                instructions: vec![Instruction::Effect {
                                    result: result.clone(),
                                    operation: operation.clone(),
                                }],
                            });
                        }
                    }
                    Operation::WithSelections { steps, action } => {
                        let saved = self.prepared.clone();
                        for step in steps {
                            let library = self.objects_library(&step.objects);
                            self.prepared.insert(step.binding.0, library);
                        }
                        self.program(&Program {
                            instructions: vec![Instruction::Effect {
                                result: result.clone(),
                                operation: (**action).clone(),
                            }],
                        });
                        self.prepared = saved;
                    }
                    Operation::FlipCoin { .. }
                    | Operation::ChoosePowerToughness { .. }
                    | Operation::DuplicateCounters { .. }
                    | Operation::DistributeCounters { .. }
                    | Operation::PutCountersEach { .. }
                    | Operation::PutChosenCounters { .. }
                    | Operation::Piles { .. }
                    | Operation::ChoosePile { .. }
                    | Operation::GiveControl { .. }
                    | Operation::AnyPlayerMayPay { .. }
                    | Operation::PlayersPayRepeatedly { .. }
                    | Operation::DelayedUnlessPaid { .. }
                    | Operation::EachPlayer { .. }
                    | Operation::Counter { .. }
                    | Operation::Populate { .. }
                    | Operation::ReduceSpeed { .. }
                    | Operation::PutCountersCapped { .. }
                    | Operation::ChooseDefender { .. }
                    | Operation::MillUntil { .. }
                    | Operation::EachObject { .. }
                    | Operation::LimitX { .. }
                    | Operation::FightEach { .. }
                    | Operation::Note(_)
                    | Operation::Bid { .. }
                    | Operation::Guess { .. }
                    | Operation::PutStickers { .. }
                    | Operation::RedistributeLife(_)
                    | Operation::PickFromHand { .. }
                    | Operation::Meld { .. } => {}
                    Operation::Seek { .. } | Operation::DrawThenDiscard { .. } => {
                        self.library = true
                    }
                    Operation::ChooseAtRandom { .. } | Operation::ChooseValue { .. } => {}
                    Operation::JoinResults { results } => {
                        let joined = results
                            .iter()
                            .filter_map(|r| self.objects.get(&r.0))
                            .fold((false, false), |a, b| (a.0 | b.0, a.1 | b.1));
                        self.objects.insert(result.0, joined);
                    }
                    Operation::AddMana { .. } => self.mana = true,
                    Operation::Draw { .. } | Operation::Mill { .. } => self.library = true,
                    Operation::Move {
                        objects,
                        destination,
                        ..
                    } => {
                        let before = self.objects_library(objects);
                        let after = Self::destination_library(&destination.address());
                        self.library |= before || after;
                        self.objects.insert(result.0, (before, after));
                    }
                    Operation::DraftChangeZone {
                        cards: objects,
                        destination,
                        ..
                    }
 => {
                        let before = self.objects_library(objects);
                        let after = Self::destination_library(destination);
                        self.library |= before || after;
                        self.objects.insert(result.0, (before, after));
                    }
                    Operation::Counter { objects, destination, .. } => {
                        let before = self.objects_library(objects);
                        let after = Self::destination_library(&destination.address());
                        self.library |= before || after;
                        self.objects.insert(result.0, (before, after));
                    }
                    Operation::ChooseObjects { objects, .. }
                    | Operation::SeparatePiles { objects, .. }
                    | Operation::NoteObjects { objects } => {
                        let library = self.objects_library(objects);
                        self.objects.insert(result.0, (library, library));
                    }
                    Operation::Search { .. } => {
                        self.objects.insert(result.0, (true, true));
                    }
                    Operation::LookAt { objects, .. }
                    | Operation::ReorderLibrary { objects, .. } => {
                        let library = self.objects_library(objects);
                        self.objects.insert(result.0, (library, library));
                    }
                    Operation::Surveil { amount, .. } => {
                        self.library |= !matches!(amount, Expr::Constant(0))
                    }
                    Operation::Reveal { .. }
                    | Operation::RevealUntil { .. }
                    | Operation::RevealUntilGroup { .. }
                    | Operation::Shuffle { .. }
                    | Operation::Scry { .. } => {}
                    Operation::PayCost { cost, .. } | Operation::PayRepeatedly { cost, .. } => {
                        self.cost(cost);
                    }
                    // Its body executes in a later triggered ability, not
                    // during this activation's resolution (CR 605.1a).
                    Operation::CreateDelayedTrigger { .. }
                    | Operation::CreateTrigger { .. }
                    | Operation::BindValue { .. }
                    | Operation::RepeatProcess
                    | Operation::DiscardObjects { .. }
                    | Operation::ReflexiveTrigger { .. }
                    | Operation::ReturnAfter { .. }
                    | Operation::CreateReplacement { .. }
                    | Operation::ModifyEvent { .. }
                    | Operation::CreateRegenerationShields { .. }
                    | Operation::Discard { .. }
                    | Operation::RemoveCounters { .. }
                    | Operation::CreateTokens { .. }
                    | Operation::Attach { .. }
                    | Operation::GainLife { .. }
                    | Operation::SetLifeTotal { .. }
                    | Operation::LoseLife { .. }
                    | Operation::DealDamage { .. }
                    | Operation::DealDamageParts { .. }
                    | Operation::DealDamageFromEach { .. }
                    | Operation::DealDamageWith { .. }
                    | Operation::Destroy { .. }
                    | Operation::SetTapped { .. }
                    | Operation::Sacrifice { .. }
                    | Operation::PutCounters { .. }
                    | Operation::CreateContinuousEffect { .. } => {}
                    // Its effects are unknown, so it must not read as a mana ability.
                    Operation::Named { .. }
                    | Operation::Primitive { .. }
                    | Operation::Keyword { .. }
                    | Operation::CreatePerpetualEffect { .. }
                    | Operation::CreateEmblem { .. }
                    | Operation::MoveCounters { .. }
                    | Operation::EachCounterKind { .. }
                    | Operation::DoubleCounters { .. }
                    | Operation::PlayCards { .. }
                    | Operation::PlayWith { .. }
                    | Operation::Together(_)
                    | Operation::RestartGame { .. }
                    | Operation::EachNumber { .. }
                    | Operation::WithReflexive { .. }
                    | Operation::Repeat { .. }
                    | Operation::TurnFaceDownAs { .. }
                    | Operation::Exchange { .. }
                    | Operation::RevealChoices { .. }
                    | Operation::FlipCoinSeries { .. }
                    | Operation::AttachEach { .. }
                    | Operation::CopyEach { .. }
                    | Operation::RetargetEach { .. }
                    | Operation::TakeSpell { .. }
                    | Operation::SetX { .. }
                    | Operation::ChangeCombat { .. }
                    | Operation::PlayersDo { .. }
                    | Operation::CreateBoon { .. }
                    | Operation::AddPhases { .. }
                    | Operation::ChooseAndPay { .. }
                    | Operation::CopyCards { .. }
                    | Operation::ChangeSpell { .. }
                    | Operation::KeywordEach { .. }
                    | Operation::MoveTopCard { .. }
                    | Operation::PayAnyAmount { .. }
                    | Operation::RevealFromHand { .. }
                    | Operation::CopyOnStack { .. }
                    | Operation::Retarget { .. }
                    | Operation::Clash { .. }
                    | Operation::Vote { .. }
                    | Operation::ChooseAction { .. }
                    | Operation::EffectOn { .. }
                    | Operation::GameEffect { .. }
                    | Operation::AttachChosen { .. }
                    | Operation::DrawUpTo { .. }
                    | Operation::SacrificeAllBut { .. }
                    | Operation::DiscardAllBut { .. }
                    | Operation::TowardPlayer { .. }
                    | Operation::Intensify { .. }
                    | Operation::FlipCoins { .. }
                    | Operation::Wish { .. }
                    | Operation::AbandonScheme { .. }
                    | Operation::PutSticker { .. }
                    | Operation::BeginOnBattlefield { .. }
                    | Operation::Manifest { .. }
                    | Operation::DraftFromSpellbook { .. }
                    | Operation::Conjure { .. } => self.library = true,
                },
                Instruction::Group(p) | Instruction::Optional { body: p, .. } => self.program(p),
                Instruction::If {
                    then, otherwise, ..
                } => {
                    self.program(then);
                    self.program(otherwise);
                }
                Instruction::RepeatableWithLimit { body, .. }
                | Instruction::ForEachObject { body, .. }
                | Instruction::ForEachPlayer { body, .. }
                | Instruction::Repeatable(body) => self.program(body),
                Instruction::ChooseOne { options, .. } => {
                    options.iter().for_each(|option| self.program(option));
                }
            }
        }
    }
    fn destination_library(destination: &Destination) -> bool {
        match destination {
            Destination::OwnersLibraryTop
            | Destination::OwnersLibraryBottom
            | Destination::OwnersLibraryFromTop(_)
            | Destination::OwnersLibraryBottomRandom
            | Destination::OwnersLibraryTopOrBottom
            | Destination::OwnersLibraryShuffled => true,
            Destination::OwnersHand
            | Destination::OwnersGraveyard
            | Destination::Exile
            | Destination::Battlefield(_)
            | Destination::OwnersBattlefield
            | Destination::Hand(_)
            | Destination::CommandZone
            | Destination::Ante
            | Destination::OwnersPlanarDeck(_) => false,
        }
    }
    fn object_library(&self, object: &ObjectRef) -> bool {
        match object {
            ObjectRef::AuraAfterEnchantedLeft { aura, host } => {
                self.object_library(aura) || self.object_library(host)
            }
            ObjectRef::Target { slot, .. } => self.targets.get(&slot.0).copied().unwrap_or(false),
            ObjectRef::Query { objects, .. } => self.objects_library(objects),
            ObjectRef::Result { result, phase, .. } => self.result_library(result, phase),
            ObjectRef::ChosenBy { result, .. } => self.result_library(result, &ObjectPhase::After),
            ObjectRef::Recorded { .. }
            | ObjectRef::Earlier { .. }
            | ObjectRef::SourceOf(_)
            | ObjectRef::Scoped
            | ObjectRef::RingBearer(_)
            | ObjectRef::HauntedBy(_)
            | ObjectRef::NthChosen { .. } => false,
            ObjectRef::Source
            | ObjectRef::AbilityHolder
            | ObjectRef::MadeHere
            | ObjectRef::GrantingObject
            | ObjectRef::Discarded
            | ObjectRef::Damage(_)
            | ObjectRef::Candidate
            | ObjectRef::AttachedTo(_)
            | ObjectRef::PairedWith(_)
            | ObjectRef::SoleTargetOf(_)
            | ObjectRef::TheOther { .. }
            | ObjectRef::AtEnd { .. }
            | ObjectRef::Affected
            | ObjectRef::EventSubject
            | ObjectRef::EventStackObject
            | ObjectRef::EventOther
            | ObjectRef::RecipientObject { .. }
            | ObjectRef::BlockPair(_)
            | ObjectRef::EventDestination
            | ObjectRef::Chosen
            | ObjectRef::Primitive { .. }
            | ObjectRef::Bound(_) => false,
        }
    }
    fn result_library(&self, result: &ResultId, phase: &ObjectPhase) -> bool {
        self.objects
            .get(&result.0)
            .is_some_and(|(before, after)| match phase {
                ObjectPhase::Before => *before,
                ObjectPhase::After => *after,
            })
    }
    fn objects_library(&self, objects: &ObjectSet) -> bool {
        match objects {
            ObjectSet::Prepared(binding) => self.prepared.get(&binding.0).copied().unwrap_or(false),
            ObjectSet::LibraryTops(_) => true,
            ObjectSet::Primitive { .. }
            | ObjectSet::Earlier { .. }
            | ObjectSet::Recorded(_)
            | ObjectSet::Scoped
            | ObjectSet::CreationLinkedCards(_)
            | ObjectSet::AnyZone(_)
            | ObjectSet::OutsideTheGame(_) => false,
            ObjectSet::Observed(result) => self.result_library(result, &ObjectPhase::Before),
            ObjectSet::Difference { objects, .. } | ObjectSet::Matching { objects, .. } => {
                self.objects_library(objects)
            }
            ObjectSet::Zone { scope, .. } => matches!(
                scope,
                ZoneScope::Players {
                    zone: EventZone::Library,
                    ..
                }
            ),
            ObjectSet::InZone { zone, .. } => matches!(zone, EventZone::Library),
            ObjectSet::One(o) => self.object_library(o),
            ObjectSet::Union(sets) => sets.iter().any(|s| self.objects_library(s)),
            ObjectSet::Take { objects, .. }
            | ObjectSet::DraftRandom { objects, .. }
            | ObjectSet::DraftChosen { objects, .. } => self.objects_library(objects),
            ObjectSet::Targets(slot) => self.targets.get(&slot.0).copied().unwrap_or(false),
            ObjectSet::Selected(result) | ObjectSet::Unselected(result) => {
                self.result_library(result, &ObjectPhase::After)
            }
            ObjectSet::Transition { result, phase } | ObjectSet::MovedBy { result, phase, .. } => {
                self.result_library(result, phase)
            }
            ObjectSet::Created(_)
            | ObjectSet::Copies(_)
            | ObjectSet::Unpaid(_)
            | ObjectSet::SplicedOnto(_)
            | ObjectSet::TargetsOf(_)
            | ObjectSet::FromHistory { .. }
            | ObjectSet::DamageSources(_) => false,
            ObjectSet::EventGroup
            | ObjectSet::Damage(_)
            | ObjectSet::CostObjects { .. }
            | ObjectSet::Affected(_) => false,
        }
    }
    fn cost(&mut self, cost: &Cost) -> bool {
        let mut loyalty = false;
        for part in &cost.parts {
            match part {
                CostPart::Primitive { .. }
                | CostPart::ManaWithX { .. }
                | CostPart::Waterbend(_)
                | CostPart::WaterbendX { .. }
                | CostPart::Teamwork(_)
                | CostPart::Keyword { .. }
                | CostPart::DiscardHand
                | CostPart::Exert(_)
                | CostPart::Unattach(_)
                | CostPart::Unspecialize(_)
                | CostPart::Behold { .. }
                | CostPart::BeholdAndExile(_)
                | CostPart::LifeEqualToOwnManaValue
                | CostPart::ManaAnyNumberOfTimes(_)
                | CostPart::ManaCostReduced { .. }
                | CostPart::RevealChoice(_)
                | CostPart::ManaXBounded { .. }
                | CostPart::UnattachFrom { .. }
                | CostPart::EnergyOneOrMore
                | CostPart::EachPlayerAction { .. }
                | CostPart::ManaReducedForEach { .. }
                | CostPart::ObjectAction { .. }
                | CostPart::ManaFromTreasure(_)
                | CostPart::SinglePlayersCards(_)
                | CostPart::MoveCounters { .. }
                | CostPart::PutCounters(_) => {}
                CostPart::ShuffleFromHand | CostPart::PlayFromTop(_) => self.library = true,
                CostPart::ExiledToGraveyard(_) => {}
                CostPart::RevealTop(_) => self.library = true,
                CostPart::Move {
                    selections,
                    destination,
                    ..
                } => {
                    self.library |= Self::destination_library(&destination.address())
                        || selections.iter().any(|s| self.objects_library(&s.objects));
                }
                CostPart::Choice(options) => {
                    for option in options {
                        loyalty |= self.cost(option);
                    }
                }
                CostPart::Constrained { part, .. } => {
                    loyalty |= self.cost(&Cost {
                        parts: vec![part.as_ref().clone()],
                    })
                }
                CostPart::Loyalty(_) => loyalty = true,
                CostPart::Mill { .. } | CostPart::Draw { .. } => self.library = true,
                CostPart::Repeated { part, .. } => {
                    loyalty |= self.cost(&Cost {
                        parts: vec![part.as_ref().clone()],
                    })
                }
                CostPart::Reveal { .. } | CostPart::PlayerAction { .. } => {}
                CostPart::RemoveCounters { .. }
                | CostPart::ManaCostOf(_)
                | CostPart::Mana(_)
                | CostPart::ChangeTapState { .. }
                | CostPart::TapSymbol
                | CostPart::Life(_)
                | CostPart::Energy(_)
                | CostPart::Sacrifice { .. }
                | CostPart::Discard { .. } => {}
            }
        }
        loyalty
    }
    fn announcement(&mut self, announcement: &Announcement) -> bool {
        for target in &announcement.targets {
            if let TargetKind::Object(domain) = &target.kind {
                self.targets
                    .insert(target.slot.0, domain.zones.contains(&EventZone::Library));
            }
        }
        let mut targets = !announcement.targets.is_empty();
        if let Some(modes) = &announcement.modes {
            for mode in &modes.options {
                targets |= self.announcement(&mode.announcement);
                self.program(&mode.body);
            }
        }
        targets
    }
}
/// CR 605.1a and 115.6, effective September 25, 2026. Even optional targets
/// require targets. Costs/effects moving cards to or from libraries disqualify
/// an activated ability; granted token abilities do not produce mana for it.
pub fn activation_kind(cost: &Cost, announcement: &Announcement, body: &Program) -> ActivationKind {
    let mut effects = Effects::default();
    let targets = effects.announcement(announcement);
    effects.program(body);
    let loyalty = effects.cost(cost);
    if effects.mana && !targets && !loyalty && !effects.library {
        ActivationKind::Mana
    } else {
        ActivationKind::Stack
    }
}
