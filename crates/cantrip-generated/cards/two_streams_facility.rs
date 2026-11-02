// Two Streams Facility
// status: planned
// requires: card type Plane
// requires: condition VotedFor
// requires: layout Planar
// requires: player rule AdditionalLandPlays
// requires: subtype Apalapucia
// requires: trigger event Player ChaosEnsues
// requires: trigger event Player Planeswalks
// requires: two choices of labels as it enters
// Whenever you planeswalk to Two Streams Facility and at the beginning of the first upkeep of the game, each player chooses green anchor or red waterfall.
// Each player who last chose green anchor may play an additional land during each of their turns.
// Creatures controlled by players who last chose red waterfall get +2/+0 and have haste.
// Whenever chaos ensues, each player who last chose green anchor chooses red waterfall, and vice versa.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let card_1 = g.card_id(&CARD); let chosen_1: u8 = cantrip_mechanics::choose::option(g, player_1, asker.source, card_1, 0..2)?;
 each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).contains(&player) }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).contains(&q.controller(affected))) }
#[allow(unused_variables)] fn ability_2_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Haste); }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_2, bound_0) in players_2.iter().copied().enumerate() { planned!("two-choices-of-labels-as-it-enters" => cantrip_mechanics::choose::value(g, asker.source, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, ChoiceKind::Word, 1, &["Green Anchor", "Red Waterfall"])?);
    } 
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-planeswalks" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Planeswalked { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true && q.history().all().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::StepBegan { step } = event { let subject = Target::Player(r.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }  None }).count() == 0 { return Some(Fired { subject, data }); } }
 None });
planned_item!("player-rule-additional-land-plays" => statics: player_rule(ability_1_0_applies, PlayerRule::ExtraLandPlays(1i64)));
planned_item!("trigger-event-player-chaos-ensues" => matcher: #[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ChaosEnsued { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Two Streams Facility", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Plane", CardType::Land => CardType::Plane)]), subtypes: SubtypeSet::of(&[planned_value!("subtype Apalapucia", Subtype::Plains => Subtype::Apalapucia)]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player Planeswalks", EventMask(0) => EventMask(EventMask::PLANESWALKED.0 | EventMask::STEP_BEGAN.0)), ability_0_event), ability_0, "Whenever you planeswalk to Two Streams Facility and at the beginning of the first upkeep of the game, each player chooses green anchor or red waterfall.").targets(&[]),triggered(when_asked(planned_value!("trigger event Player ChaosEnsues", EventMask(0) => EventMask(EventMask::CHAOS_ENSUED.0)), ability_3_event), ability_3, "Whenever chaos ensues, each player who last chose green anchor chooses red waterfall, and vice versa.").targets(&[])], statics: &[static_ability(Layer::Ability, ability_2_affects, ability_2_apply_1, "Creatures controlled by players who last chose red waterfall get +2/+0 and have haste."),static_ability(Layer::PtModify, ability_2_affects, ability_2_apply_0, "Creatures controlled by players who last chose red waterfall get +2/+0 and have haste.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Green Anchor", "Red Waterfall"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
