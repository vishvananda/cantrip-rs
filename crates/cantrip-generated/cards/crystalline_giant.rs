// Crystalline Giant
// status: planned
// requires: operation PutChosenCounters
// At the beginning of combat on your turn, choose a kind of counter at random that this creature doesn't have on it from among flying, first strike, deathtouch, hexproof, lifelink, menace, reach, trample, vigilance, and +1/+1. Put a counter of that kind on this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-put-chosen-counters" => cantrip_mechanics::counters::chosen(g, asker, { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }, "RandomMissing([Named(FlyingCounter), Named(FirstStrikeCounter), Named(DeathtouchCounter), Named(HexproofCounter), Named(LifelinkCounter), Named(MenaceCounter), Named(ReachCounter), Named(TrampleCounter), Named(VigilanceCounter), PowerToughness(1, 1)])")?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Crystalline Giant", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, choose a kind of counter at random that this creature doesn't have on it from among flying, first strike, deathtouch, hexproof, lifelink, menace, reach, trample, vigilance, and +1/+1. Put a counter of that kind on this creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
