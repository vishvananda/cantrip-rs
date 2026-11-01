// Maralen of the Mornsong Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: numeric expression Game
// requires: trigger event Player GameBegins
// At the beginning of the game, you may pay any amount of life.
// You can't draw cards.
// At the beginning of your draw step, look at the top X cards of your library, where X is the amount of life paid with Maralen of the Mornsong Avatar. Put one of them into your hand, then shuffle.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { let most = g.state().player(player).life.clamp(0, 255) as u32; let paid = cantrip_mechanics::choose::number(g, player, asker.source, 0, most, "Pay")?; if paid > 0 && g.may_pay(player, asker.source, asker.source, life(paid as u8), &PAY_QUESTION)? { i64::from(paid) } else { 0 } }; }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, n: u32) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Draw) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_3 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((planned_value!("numeric expression Game", 0i64 => q.game_value(item, GameValue::LifePaid))).max(0) as usize).collect::<Vec<ObjectId>>() }; g.look_at(player_3, &objects_3);
let selected_4_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; (objects_3.clone()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Library).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::PutIntoHand, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
let objects_5 = { let q: &dyn Query = g; selected_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.shuffle(player); }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("trigger-event-player-game-begins" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::GameBegan { player } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if true { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: -3 }));

pub static CARD: CardDef = CardDef { name: "Maralen of the Mornsong Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player GameBegins", EventMask(0) => EventMask(EventMask::GAME_BEGAN.0)), ability_0_event), ability_0, "At the beginning of the game, you may pay any amount of life.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), ability_2, "At the beginning of your draw step, look at the top X cards of your library, where X is the amount of life paid with Maralen of the Mornsong Avatar. Put one of them into your hand, then shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cant_draw: &[ability_1_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
