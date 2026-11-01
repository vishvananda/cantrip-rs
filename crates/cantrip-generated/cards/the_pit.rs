// The Pit
// status: planned
// requires: card type Plane
// requires: layout Planar
// requires: operation ChooseAction
// requires: subtype TheAbyss
// requires: trigger event Player ChaosEnsues
// requires: trigger event Player Planeswalks
// When you planeswalk to The Pit, each player creates their choice of a 3/3 white Angel creature token with flying or a 6/6 black Demon creature token with flying, trample, and "At the beginning of your upkeep, sacrifice another creature. If you can't, this token deals 6 damage to you."
// Whenever chaos ensues, each player sacrifices a nonartifact creature of their choice.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-action" => cantrip_mechanics::choose::action(g, item, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, &[&[CreatableToken::TokenWithPT(PT::PT(3, 3), TokenColorList::Colors(SimpleColor::White), &[], &[CardType::Creature], TokenSubtypes::SubtypesList(SubType::Angel), &["Ability(Keyword(Flying))"])], &[], &[CreatableToken::TokenWithPT(PT::PT(6, 6), TokenColorList::Colors(SimpleColor::Black), &[], &[CardType::Creature], TokenSubtypes::SubtypesList(SubType::Demon), &["Ability(Keyword(Flying))", "Ability(Keyword(Trample))", "Ability(Triggered { event: Turn { point: UpkeepBegins, players: One(AbilityController) }, intervening_if: None, limits: TriggerLimits { per_turn: None, total: None }, announcement: Announcement { targets: [], modes: None }, body: Program { instructions: [Effect { result: ResultId(0), operation: PayCost { player: AbilityController, cost: Cost { parts: [Sacrifice { objects: Battlefield(All([Not(SameObject(Source)), CardType(Creature)])), choose: Some(Exactly(Constant(1))) }] }, optional: false } }, If { condition: Not(PaymentChosen(ResultId(0))), then: Program { instructions: [Effect { result: ResultId(1), operation: DealDamage { source: Source, recipients: Players(One(AbilityController)), amount: Constant(6), preventable: true } }] }, otherwise: Program { instructions: [] } }] } })"])], &[]], false)?);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Artifact)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let players_1 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_1: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_1 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_1 { let eligible: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_1.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_1.clone(), &[]))? }; for object in taken { picks_1.push((player, object)); } }
let sacrificed_1 = g.sacrifice_each(&picks_1);
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-planeswalks" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Planeswalked { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "The Pit", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype TheAbyss", Subtype::Plains => Subtype::TheAbyss)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player Planeswalks", EventMask(0) => EventMask(EventMask::PLANESWALKED.0)), ability_0_event), ability_0, "When you planeswalk to The Pit, each player creates their choice of a 3/3 white Angel creature token with flying or a 6/6 black Demon creature token with flying, trample, and \"At the beginning of your upkeep, sacrifice another creature. If you can't, this token deals 6 damage to you.\"").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_1_event), ability_1, "Whenever chaos ensues, each player sacrifices a nonartifact creature of their choice.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
