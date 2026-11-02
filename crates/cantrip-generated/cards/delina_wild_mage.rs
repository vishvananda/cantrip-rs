// Delina, Wild Mage
// status: ready
// Whenever Delina attacks, choose target creature you control, then roll a d20.
// 1—14 | Create a tapped and attacking token that's a copy of that creature, except it's not legendary and it has "At end of combat, exile this token."
// 15—20 | Create one of those tokens. You may roll again.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn granted_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::EndOfCombat) && true { return Some(Fired { subject, data }); } }
 None }
fn granted_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_4 = g.exile_all(&objects_4);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn granted_7_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::EndOfCombat) && true { return Some(Fired { subject, data }); } }
 None }
fn granted_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_8 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_8 = g.exile_all(&objects_8);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
loop { let mut again_0 = false;
let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_1 = i64::from(g.roll_die(player_1, 20).result);
g.end_sentence();
if { let q: &dyn Query = g; ((value_1) >= (1i64) && (value_1) <= (14i64)) } {
let original_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let values_2 = { let q: &dyn Query = g; original_2.map(|original: ObjectId| q.copiable_values(original).nonlegendary().with_ability(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_2 = match values_2 { Some(values) => g.create_token_copies_tapped(values, player_2, count_2, true)?, None => Vec::new() };
for &token in &created_2 { let controller = g.controller(token); if let Some(defender) = cantrip_mechanics::combat::choose_attack_target(g, controller, asker.source)? { g.enter_attacking(&[token], defender); } }
g.end_sentence();
hoisted_5.extend(created_2.clone());
}
if { let q: &dyn Query = g; ((value_1) >= (15i64) && (value_1) <= (20i64)) } {
let original_6 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let values_6 = { let q: &dyn Query = g; original_6.map(|original: ObjectId| q.copiable_values(original).nonlegendary().with_ability(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 1))) }; let count_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_6 = match values_6 { Some(values) => g.create_token_copies_tapped(values, player_6, count_6, true)?, None => Vec::new() };
for &token in &created_6 { let controller = g.controller(token); if let Some(defender) = cantrip_mechanics::combat::choose_attack_target(g, controller, asker.source)? { g.enter_attacking(&[token], defender); } }
g.end_sentence();
let decider_9 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_9, asker.source, Purpose::Optional)? {
again_0 = true;
g.end_sentence();
}
g.end_sentence();
hoisted_11.extend(created_6.clone());
}
g.end_sentence();
hoisted_12.extend(hoisted_5.clone());
hoisted_13.extend(hoisted_11.clone());
if !again_0 { break; } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Delina, Wild Mage", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_0_event), ability_0, "Whenever Delina attacks, choose target creature you control, then roll a d20.").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_3_event), granted_3, "At the beginning of this step, Return the chosen permanents to their owners' hands.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), granted_7_event), granted_7, "At the beginning of this step, Return the chosen permanents to their owners' hands.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
