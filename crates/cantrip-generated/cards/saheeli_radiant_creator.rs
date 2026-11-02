// Saheeli, Radiant Creator
// status: ready
// Whenever you cast an Artificer or artifact spell, you get {E} (an energy counter).
// At the beginning of combat on your turn, you may pay {E}{E}{E}. When you do, create a token that's a copy of target permanent you control, except it's a 5/5 artifact creature in addition to its other types and has haste. Sacrifice it at the beginning of the next end step.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Artificer)) || q.chars(id).is(CardType::Artifact)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; g.change_energy(player_0, amount_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (Some(asker.controller) == Some(q.controller(id)))) }
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_4_target_0)];
fn delayed_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_8 = { let q: &dyn Query = g; about(q, item, 0) }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_8: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_8 { let controller = g.controller(object); if players_8.contains(&controller) && g.can_sacrifice(controller, object) { picks_8.push((controller, object)); } }
let sacrificed_8 = g.sacrifice_each(&picks_8);
g.end_sentence();
 Ok(()) }
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_5 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let values_5 = { let q: &dyn Query = g; original_5.map(|original: ObjectId| q.copiable_values(original).sized(5, 5).with_type(CardType::Artifact).with_type(CardType::Creature).with_keyword(Keyword::Haste)) }; let count_5 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_5 = match values_5 { Some(values) => g.create_token_copies_tapped(values, player_5, count_5, false)?, None => Vec::new() };
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_5.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if g.state().player(decider_1).energy() >= 3 && cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let owed_2: u8 = { let q: &dyn Query = g; (3i64).max(0) as u8 }; let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_2 = g.state().player(payer_2).energy() >= owed_2; if paid_2 { g.change_energy(payer_2, -i32::from(owed_2)); }
if paid_2 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Saheeli, Radiant Creator", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast an Artificer or artifact spell, you get {E} .").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of combat on your turn, you may pay {E}{E}{E}. When you do, create a token that's a copy of target permanent you control, except it's a 5/5 artifact creature in addition to its other types and has haste. Sacrifice it at the beginning of the next end step.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_7, "When you do, create a token that's a copy of target permanent you control, except it's a 5/5 artifact creature in addition to its other types and has haste."),delayed_trigger(reflexive_4, "When you do, create a token that's a copy of target permanent you control, except it's a 5/5 artifact creature in addition to its other types and has haste. Sacrifice it at the beginning of the next end step.").targets(REFLEXIVE_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
