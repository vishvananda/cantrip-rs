// Shadrix Silverquill
// status: planned
// requires: modes that may all be declined
// requires: modes with distinct targets
// Flying, double strike
// At the beginning of combat on your turn, you may choose two. Each mode must target a different player.
// • Target player creates a 2/1 white and black Inkling creature token with flying.
// • Target player draws a card and loses 1 life.
// • Target player puts a +1/+1 counter on each creature they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn ability_2_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_2_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_mode_0_target_0)];
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Inkling).name, token_words: token!(Inkling).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Inkling]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(5)), power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let players_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_2_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_mode_1_target_0)];
fn ability_2_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_3 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_mode_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_2_MODE_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_mode_2_target_0)];
fn ability_2_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::PlusOne, amount)); } }
let had_4: Vec<_> = counters_4.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_4); let applied_4: Vec<ObjectId> = counters_4.iter().zip(had_4).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
static ABILITY_2_MODES: ModalDef = planned_value!("modes with distinct targets", planned_value!("modes that may all be declined", modes(&[mode(ability_2_mode_0, "Target player creates a 2/1 white and black Inkling creature token with flying.").targets(ABILITY_2_MODE_0_TARGETS),mode(ability_2_mode_1, "Target player draws a card and loses 1 life.").targets(ABILITY_2_MODE_1_TARGETS),mode(ability_2_mode_2, "Target player puts a +1/+1 counter on each creature they control.").targets(ABILITY_2_MODE_2_TARGETS)]).choose(2, 2) => modes(&[mode(ability_2_mode_0, "Target player creates a 2/1 white and black Inkling creature token with flying.").targets(ABILITY_2_MODE_0_TARGETS),mode(ability_2_mode_1, "Target player draws a card and loses 1 life.").targets(ABILITY_2_MODE_1_TARGETS),mode(ability_2_mode_2, "Target player puts a +1/+1 counter on each creature they control.").targets(ABILITY_2_MODE_2_TARGETS)]).choose(2, 2).may_decline()) => planned_value!("modes that may all be declined", modes(&[mode(ability_2_mode_0, "Target player creates a 2/1 white and black Inkling creature token with flying.").targets(ABILITY_2_MODE_0_TARGETS),mode(ability_2_mode_1, "Target player draws a card and loses 1 life.").targets(ABILITY_2_MODE_1_TARGETS),mode(ability_2_mode_2, "Target player puts a +1/+1 counter on each creature they control.").targets(ABILITY_2_MODE_2_TARGETS)]).choose(2, 2) => modes(&[mode(ability_2_mode_0, "Target player creates a 2/1 white and black Inkling creature token with flying.").targets(ABILITY_2_MODE_0_TARGETS),mode(ability_2_mode_1, "Target player draws a card and loses 1 life.").targets(ABILITY_2_MODE_1_TARGETS),mode(ability_2_mode_2, "Target player puts a +1/+1 counter on each creature they control.").targets(ABILITY_2_MODE_2_TARGETS)]).choose(2, 2).may_decline()).distinct_targets());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Shadrix Silverquill", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elder,Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::DoubleStrike]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_2_event), &ABILITY_2_MODES, "At the beginning of combat on your turn, you may choose two. Each mode must target a different player. Target player creates a 2/1 white and black Inkling creature token with flying; or Target player draws a card and loses 1 life; or Target player puts a +1/+1 counter on each creature they control.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
