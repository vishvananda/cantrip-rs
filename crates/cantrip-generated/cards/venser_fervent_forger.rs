// Venser, Fervent Forger
// status: ready
// Flash
// When Venser enters, choose one —
// • Copy target instant or sorcery spell an opponent controls twice. You may choose new targets for the copies.
// • Create two tokens that are copies of target permanent an opponent controls. They gain haste. At the beginning of the next end step, sacrifice them.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn ability_1_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_1_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_0_target_0)];
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let runs_0: u32 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let spell_0: Option<ObjectId> = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied();
if let Some(spell) = spell_0 { for _ in 0..runs_0 { g.copy_spell(spell, copier_0, true)?; } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)))) }
static ABILITY_1_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_1_target_0)];
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; about(q, item, 0) }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
 Ok(()) }
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; let count_1 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_1 = match original_1 { Some(original) => g.create_token_copies(original, player_1, count_1)?, None => Vec::new() };
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (created_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::Indefinite;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2);  } }); }
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_1.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: None }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = modes(&[mode(ability_1_mode_0, "Copy target instant or sorcery spell an opponent controls twice. You may choose new targets for the copies.").targets(ABILITY_1_MODE_0_TARGETS),mode(ability_1_mode_1, "Create two tokens that are copies of target permanent an opponent controls. They gain haste. At the beginning of the next end step, sacrifice them.").targets(ABILITY_1_MODE_1_TARGETS)]).choose(1, 1);
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Venser, Fervent Forger", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Sorcerer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[modal_triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), &ABILITY_1_MODES, "When Venser enters, choose one — Copy target instant or sorcery spell an opponent controls twice. You may choose new targets for the copies; or Create two tokens that are copies of target permanent an opponent controls. They gain haste. At the beginning of the next end step, sacrifice them.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, "At the beginning of the next end step, sacrifice them.")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
