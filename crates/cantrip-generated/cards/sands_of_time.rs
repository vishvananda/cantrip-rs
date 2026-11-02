// Sands of Time
// status: planned
// requires: skipping a step of the turn
// Each player skips their untap step.
// At the beginning of each player's upkeep, that player simultaneously untaps each tapped artifact, creature, and land they control and taps each untapped artifact, creature, and land they control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let turned_0_0: Vec<ObjectId> = ({ let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Land)) && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o| g.state().obj(o).tapped != true).collect();
let turned_0_1: Vec<ObjectId> = ({ let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.state().obj(id).tapped && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Land)) && item.subject.player() == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o| g.state().obj(o).tapped != false).collect();
cantrip_api::instruction(g, |g| { g.tap_together(&turned_0_0); for &object in &turned_0_1 { g.untap(object); } });
g.end_sentence();
 Ok(()) }
planned_item!("skipping-a-step-of-the-turn" => skip_steps: SkipStep { applies: |q, asker, player| true, step: Step::Untap });

pub static CARD: CardDef = CardDef { name: "Sands of Time", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each player's upkeep, that player simultaneously untaps each tapped artifact, creature, and land they control and taps each untapped artifact, creature, and land they control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
