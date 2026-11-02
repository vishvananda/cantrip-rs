// V.A.T.S.
// status: ready
// Split second (As long as this spell is on the stack, players can't cast spells or activate abilities that aren't mana abilities.)
// Choose any number of target creatures with equal toughness. Destroy the chosen creatures.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_api::select::set_part(&(common_toughness as SelectFn), &(largest_common_toughness as cantrip_api::select::LargestFn), 0, "with equal toughness"))];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
fn toughness_of(q: &dyn Query, id: ObjectId) -> i16 { q.chars(id).toughness.unwrap_or(0) }
fn common_toughness(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let first = choice.selected.first().map(|&c| toughness_of(q, c)); Verdict { complete: choice.selected.iter().all(|&c| Some(toughness_of(q, c)) == first), addable: Bits::of(choice.candidates.len(), |i| first.map_or(true, |k| toughness_of(q, choice.candidates[i]) == k)) } }
fn largest_common_toughness(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { choice.candidates.iter().map(|&a| choice.candidates.iter().copied().filter(|&b| toughness_of(q, b) == toughness_of(q, a)).collect::<Vec<_>>()).max_by_key(Vec::len).unwrap_or_default() }

pub static CARD: CardDef = CardDef { name: "V.A.T.S.", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::SplitSecond]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
