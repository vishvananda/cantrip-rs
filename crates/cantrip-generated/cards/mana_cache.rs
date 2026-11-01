// Mana Cache
// status: planned
// requires: engine: no activation by other players
// requires: turn window BeforeEndStep
// At the beginning of each player's end step, put a charge counter on this enchantment for each untapped land that player controls.
// Remove a charge counter from this enchantment: Add {C}. Any player may activate this ability but only during their turn before the end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Land) && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 32767) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::Charge, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::C, mana_1_0);}
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { (Some(asker.controller) == Some(q.state().turn.active) && planned_value!("turn window BeforeEndStep", false => q.state().turn.is_in(TurnWindow::BeforeEndStep))) }

pub static CARD: CardDef = CardDef { name: "Mana Cache", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of each player's end step, put a charge counter on this enchantment for each untapped land that player controls.").targets(&[]),planned_value!("engine: no activation by other players", mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(remove_counters(CounterKind::Charge, 1)), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_1_mana) }, "Remove a charge counter from this enchantment: Add {C}. Any player may activate this ability but only during their turn before the end step.").condition(ability_1_if) => mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(remove_counters(CounterKind::Charge, 1)), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_1_mana) }, "Remove a charge counter from this enchantment: Add {C}. Any player may activate this ability but only during their turn before the end step.").condition(ability_1_if).activated_by(|q, asker, player| true))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
