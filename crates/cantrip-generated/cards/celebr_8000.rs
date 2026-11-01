// Celebr-8000
// status: planned
// requires: keyword action RollDie(6) with an amount
// requires: numeric expression Dice
// At the beginning of combat on your turn, roll two six-sided dice. For each result of 1, this creature gets +1/+1 until end of turn. For each other result, it gains the indicated ability until end of turn. If you rolled doubles, it also gains double strike until end of turn.
// • 2 — menace
// • 3 — vigilance
// • 4 — lifelink
// • 5 — flying
// • 6 — indestructible
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0: i64 = planned_value!("keyword action RollDie(6) with an amount", 0i64 => cantrip_mechanics::dice::roll_several(g, player_0, 6, amount_0)?.total());
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Pump((planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 1i64))).clamp(-32768, 32767) as i16, (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 1i64))).clamp(-32768, 32767) as i16) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 2i64))) >= (1i64) } {
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Menace])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
hoisted_3.extend(objects_2.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 3i64))) >= (1i64) } {
let objects_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfTurn;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Vigilance])) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
hoisted_5.extend(objects_4.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 4i64))) >= (1i64) } {
let objects_6 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let duration_6 = Duration::EndOfTurn;
let kind_6_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])) };
if let Some(source) = source_6 { cantrip_api::instruction(g, |g| { for &object in &objects_6 { g.add_effect(source, object, kind_6_0_0, duration_6);  } }); }
g.end_sentence();
hoisted_7.extend(objects_6.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 5i64))) >= (1i64) } {
let objects_8 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_8 = { let q: &dyn Query = g; Some(asker.source) }; let duration_8 = Duration::EndOfTurn;
let kind_8_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying])) };
if let Some(source) = source_8 { cantrip_api::instruction(g, |g| { for &object in &objects_8 { g.add_effect(source, object, kind_8_0_0, duration_8);  } }); }
g.end_sentence();
hoisted_9.extend(objects_8.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).matching(Comparison::Equal, 6i64))) >= (1i64) } {
let objects_10 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_10 = { let q: &dyn Query = g; Some(asker.source) }; let duration_10 = Duration::EndOfTurn;
let kind_10_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Indestructible])) };
if let Some(source) = source_10 { cantrip_api::instruction(g, |g| { for &object in &objects_10 { g.add_effect(source, object, kind_10_0_0, duration_10);  } }); }
g.end_sentence();
hoisted_11.extend(objects_10.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; (planned_value!("numeric expression Dice", 0i64 => g.last_roll(asker.controller).most_equal())) == (2i64) } {
let objects_12 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_12 = { let q: &dyn Query = g; Some(asker.source) }; let duration_12 = Duration::EndOfTurn;
let kind_12_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::DoubleStrike])) };
if let Some(source) = source_12 { cantrip_api::instruction(g, |g| { for &object in &objects_12 { g.add_effect(source, object, kind_12_0_0, duration_12);  } }); }
g.end_sentence();
hoisted_13.extend(objects_12.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Celebr-8000", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Clown,Subtype::Robot]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of combat on your turn, roll two six-sided dice. For each result of 1, this creature gets +1/+1 until end of turn. For each other result, it gains the indicated ability until end of turn. If you rolled doubles, it also gains double strike until end of turn. 2 — menace; or 3 — vigilance; or 4 — lifelink; or 5 — flying; or 6 — indestructible.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
