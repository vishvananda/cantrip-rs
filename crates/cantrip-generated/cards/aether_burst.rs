// Aether Burst
// status: ready
// Return up to X target creatures to their owners' hands, where X is one plus the number of cards named Aether Burst in all graveyards as you cast this spell.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_up_to_most(q: &dyn Query, source: ObjectId, chooser: PlayerId, x: u32) -> usize { let asker = &Asker::new(source, chooser); (((1i64 + (((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).name == "Aether Burst").collect::<Vec<ObjectId>>()).len() as i64)).min(u32::MAX.into()) as u32 as usize).max(slot_held(q, source, 0)) }
fn ability_0_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Verdict { let most = ability_0_0_up_to_most(q, choice.source, choice.chooser, x); Verdict { complete: choice.selected.len() <= most, addable: Bits::new(choice.candidates.len(), choice.selected.len() < most) } }
fn largest_ability_0_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Vec<ObjectId> { choice.candidates.iter().copied().take(ability_0_0_up_to_most(q, choice.source, choice.chooser, x)).collect() }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_api::select::set_part(&(ability_0_0_up_to as SelectFn), &(largest_ability_0_0_up_to as cantrip_api::select::LargestFn), 0, "up to X"))];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn slot_held(q: &dyn Query, source: ObjectId, slot: usize) -> usize { let s = q.state(); s.stack.iter().chain(s.left_stack.iter()).filter(|i| i.object == source).map(|i| s.targets(i)[s.spec_targets(i, slot)].iter().filter(|t| **t != Target::None).count()).max().unwrap_or(0) }

pub static CARD: CardDef = CardDef { name: "Aether Burst", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
