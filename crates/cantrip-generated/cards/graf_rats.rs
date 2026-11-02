// Graf Rats
// status: planned
// requires: operation Meld
// At the beginning of combat on your turn, if you both own and control this creature and a creature named Midnight Scavengers, exile them, then meld them into Chittering Host.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_if(q: &dyn Query, asker: &Asker) -> bool { (Some(asker.source).is_some_and(|candidate: ObjectId| (Some(asker.controller) == Some(q.state().obj(candidate).owner) && Some(asker.controller) == Some(q.controller(candidate)))) && ((q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) && q.chars(id).name == "Midnight Scavengers") && Some(asker.controller) == Some(q.state().obj(id).owner) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (1i64)) }
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).name == "Midnight Scavengers")).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::ChooseAsEnters, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
let objects_1 = { let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in Some(asker.source).into_iter().collect::<Vec<ObjectId>>() { if !all.contains(&id) { all.push(id); } } for id in selected_0.clone() { if !all.contains(&id) { all.push(id); } } all } }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
let melding_3 = { let q: &dyn Query = g; moved_1.clone() };
planned!("operation-meld" => g.meld(&melding_3, "Chittering Host"););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Graf Rats", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, if you both own and control this creature and a creature named Midnight Scavengers, exile them, then meld them into Chittering Host.").targets(&[]).condition(ability_0_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
