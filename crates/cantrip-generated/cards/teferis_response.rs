// Teferi's Response
// status: planned
// requires: modes implied by the target chosen
// Counter target spell or ability an opponent controls that targets a land you control. If a permanent's ability is countered this way, destroy that permanent.
// Draw two cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)) && q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == id).is_some_and(|i| q.state().targets(i).iter().any(|t| t.object().is_some_and(|o| (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).contains(&o))))))) }
static ABILITY_0_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_0_target_0)];
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; cantrip_api::instruction(g, |g| { for spell in objects_0 { g.counter(spell); } });
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { q.state().stack.iter().any(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false }) && (((Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&e.controller) && q.state().targets(e).iter().any(|t| t.object().is_some_and(|o| (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).contains(&o)))))) }
static ABILITY_0_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_mode_1_target_0)];
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let items_2: Vec<(ObjectId, usize)> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).into_iter().collect::<Vec<Target>>().into_iter().filter_map(|t| { let stack = &g.state().stack; stack.iter().position(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false })).map(|at| (stack[at].object, at)) }).collect(); let abilities_2: Vec<ObjectId> = items_2.iter().filter(|&&(_, at)| g.state().stack[at].kind != StackKind::Spell).map(|&(o, _)| o).filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect(); let spells_2: Vec<ObjectId> = items_2.iter().filter(|&&(_, at)| g.state().stack[at].kind == StackKind::Spell).map(|&(o, _)| o).collect(); let countered_2 = g.counter_items(&items_2); let mut moved_2: Vec<ObjectId> = if countered_2 > 0 { abilities_2 } else { Vec::new() }; moved_2.extend(spells_2.into_iter().filter_map(|o| g.state().obj(o).became));
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (moved_2.clone()).first().copied().into_iter().collect::<Vec<ObjectId>>() }; let destroyed_3 = g.destroy_all(&objects_3);
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
 Ok(()) }
static ABILITY_0_MODES: ModalDef = planned_value!("modes implied by the target chosen", modes(&[mode(ability_0_mode_0, " Draw 2 cards.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, " Destroy the chosen permanents. Draw 2 cards.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_0_mode_0, " Draw 2 cards.").targets(ABILITY_0_MODE_0_TARGETS),mode(ability_0_mode_1, " Destroy the chosen permanents. Draw 2 cards.").targets(ABILITY_0_MODE_1_TARGETS)]).choose(1, 1).implied_by_target());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Teferi's Response", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: Some(SpellDef::MODAL), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: Some(&ABILITY_0_MODES), enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
