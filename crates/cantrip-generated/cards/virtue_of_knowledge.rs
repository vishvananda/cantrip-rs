// Virtue of Knowledge // Vantress Visions
// status: ready
// If a permanent entering causes a triggered ability of a permanent you control to trigger, that ability triggers an additional time.
// Copy target activated or triggered ability you control. You may choose new targets for the copy.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_trigger_rule(q: &dyn Query, asker: &Asker, event: &Event, triggering: &Triggering) -> bool { true && (triggering.permanent && Some(asker.controller) == Some(triggering.controller)) && cantrip_mechanics::statics::entering(event).is_some_and(|id| true) }

pub static CARD: CardDef = CardDef { name: "Virtue of Knowledge", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_triggers: &[ability_0_trigger_rule], adventure: Some(&adventure::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&adventure::CARD];
pub mod adventure {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { q.state().stack.iter().any(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false }) && (((e.kind == StackKind::Activated || e.kind == StackKind::Triggered) && Some(asker.controller) == Some(e.controller)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let copier_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if let Some(t) = (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).into_iter().collect::<Vec<Target>>()).first().copied() { match t { Target::Object(spell) => { g.copy_spell(spell, copier_0, true)?; } t => { g.copy_ability(t, copier_0, true)?; } } }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Vantress Visions", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[Subtype::Adventure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
