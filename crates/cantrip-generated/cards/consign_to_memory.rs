// Consign to Memory
// status: ready
// Replicate {1} (When you cast this spell, copy it for each time you paid its replicate cost. You may choose new targets for the copies.)
// Counter target triggered ability or colorless spell.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { q.state().stack.iter().any(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false }) && (((e.kind == StackKind::Spell && (e.kind == StackKind::Spell && q.chars(e.object).colors.is_empty())) || e.kind == StackKind::Triggered))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let items_0: Vec<(ObjectId, usize)> = t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).into_iter().collect::<Vec<Target>>().into_iter().filter_map(|t| { let stack = &g.state().stack; stack.iter().position(|e| (match t { Target::Object(id) => e.kind == StackKind::Spell && e.object == id, Target::Ability(serial) => e.kind != StackKind::Spell && e.serial == serial, _ => false })).map(|at| (stack[at].object, at)) }).collect(); g.counter_items(&items_0);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Consign to Memory", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[cantrip_mechanics::replicate::trigger("Replicate")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: multikicker(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
