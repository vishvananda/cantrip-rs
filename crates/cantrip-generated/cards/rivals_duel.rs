// Rivals' Duel
// status: ready
// Choose two target creatures that share no creature types. Those creatures fight each other. (Each deals damage equal to its power to the other.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(2, 2).together(cantrip_api::select::set_part(&(no_common_creature_type as SelectFn), &(largest_no_common_creature_type as cantrip_api::select::LargestFn), 0, "that share no creature types"))];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let first_0 = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied(); let second_0 = ({ let q: &dyn Query = g; (t_0_raw.get(1).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied();
if let (Some(first), Some(second)) = (first_0, second_0) { g.fight(first, second); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn creature_types_of(q: &dyn Query, id: ObjectId) -> SubtypeSet { SubtypeSet::CREATURE_TYPES.minus(SubtypeSet::CREATURE_TYPES.minus(q.chars(id).subtypes)) }
fn apart_in_type(q: &dyn Query, a: ObjectId, b: ObjectId) -> bool { !creature_types_of(q, a).intersects(creature_types_of(q, b)) }
fn no_common_creature_type(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let selected = choice.selected; Verdict { complete: selected.iter().enumerate().all(|(i, &a)| selected[..i].iter().all(|&b| apart_in_type(q, a, b))), addable: Bits::of(choice.candidates.len(), |i| selected.iter().all(|&b| apart_in_type(q, choice.candidates[i], b))) } }
fn grow_apart_in_type(q: &dyn Query, left: &[ObjectId], have: &mut Vec<ObjectId>, best: &mut Vec<ObjectId>) { if have.len() > best.len() { *best = have.clone(); } if have.len() + left.len() <= best.len() { return; } for (i, &c) in left.iter().enumerate() { if have.iter().all(|&o| apart_in_type(q, o, c)) { have.push(c); grow_apart_in_type(q, &left[i + 1..], have, best); have.pop(); } } }
fn largest_no_common_creature_type(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { let mut best = Vec::new(); grow_apart_in_type(q, choice.candidates, &mut Vec::new(), &mut best); best }

pub static CARD: CardDef = CardDef { name: "Rivals' Duel", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
