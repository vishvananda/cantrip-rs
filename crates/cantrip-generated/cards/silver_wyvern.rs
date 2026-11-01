// Silver Wyvern
// status: planned
// requires: modes implied by the target chosen
// Flying
// {U}: Change the target of target spell or ability that targets only this creature. The new target must be a creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_mode_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && ((q.state().stack.iter().find(|i| i.object == id).is_some_and(|i| q.state().targets(i).len() == 1) && q.state().stack.iter().find(|i| i.kind == StackKind::Spell && i.object == id).is_some_and(|i| q.state().targets(i).iter().any(|t| t.object().is_some_and(|o| (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).contains(&o))))))) }
static ABILITY_1_MODE_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_0_target_0)];
fn ability_1_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let chooser_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let retarget_0: Option<Target> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)); if let Some(t) = retarget_0 {
g.change_target(t, chooser_0)?;
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_mode_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { q.state().stack.iter().any(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false }) && ((e.target_count == 1 && q.state().targets(e).iter().any(|t| t.object().is_some_and(|o| (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).contains(&o)))))) }
static ABILITY_1_MODE_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_mode_1_target_0)];
fn ability_1_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let chooser_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let retarget_1: Option<Target> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)); if let Some(t) = retarget_1 {
g.change_target(t, chooser_1)?;
}
g.end_sentence();
 Ok(()) }
static ABILITY_1_MODES: ModalDef = planned_value!("modes implied by the target chosen", modes(&[mode(ability_1_mode_0, "").targets(ABILITY_1_MODE_0_TARGETS),mode(ability_1_mode_1, "").targets(ABILITY_1_MODE_1_TARGETS)]).choose(1, 1) => modes(&[mode(ability_1_mode_0, "").targets(ABILITY_1_MODE_0_TARGETS),mode(ability_1_mode_1, "").targets(ABILITY_1_MODE_1_TARGETS)]).choose(1, 1).implied_by_target());
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Silver Wyvern", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Drake]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[modal_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &ABILITY_1_MODES, "{U}: Change the target of target spell or ability that targets only this creature. The new target must be a creature.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
