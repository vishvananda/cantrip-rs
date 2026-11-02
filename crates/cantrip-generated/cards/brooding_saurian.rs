// Brooding Saurian
// status: ready
// At the beginning of each end step, each player gains control of all nontoken permanents they own.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let snap_1: Vec<Vec<ObjectId>> = players_0.iter().copied().map(|bound_0| { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).token) && Some(bound_0) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() }).collect();
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
g.one_instruction(&mut |g| { for (index_0, bound_0) in players_0.iter().copied().enumerate() { let objects_2 = { let q: &dyn Query = g; (snap_1[index_0].clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(bound_0).expect("a reference that always names a player")) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
 each_3[index_0] = objects_2.clone(); all_3.extend(each_3[index_0].iter().copied());
   } Ok(()) })?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Brooding Saurian", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of each end step, each player gains control of all nontoken permanents they own.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
