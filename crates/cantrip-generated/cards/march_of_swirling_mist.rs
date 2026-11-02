// March of Swirling Mist
// status: ready
// As an additional cost to cast this spell, you may exile any number of blue cards from your hand. This spell costs {2} less to cast for each card exiled this way.
// Up to X target creatures phase out. (While they're phased out, they're treated as though they don't exist. Each one phases in before its controller untaps during their next untap step.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) }
#[allow(unused_variables)] fn ability_1_0_up_to_most(q: &dyn Query, source: ObjectId, chooser: PlayerId, x: u32) -> usize { let asker = &Asker::new(source, chooser); (i64::from(x)) as u32 as usize }
fn ability_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Verdict { let most = ability_1_0_up_to_most(q, choice.source, choice.chooser, x); Verdict { complete: choice.selected.len() <= most, addable: Bits::new(choice.candidates.len(), choice.selected.len() < most) } }
fn largest_ability_1_0_up_to(q: &dyn Query, choice: &Choice, x: u32) -> Vec<ObjectId> { choice.candidates.iter().copied().take(ability_1_0_up_to_most(q, choice.source, choice.chooser, x)).collect() }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0).between(0, cantrip_api::TargetSpec::ANY_NUMBER).together(cantrip_api::select::set_part(&(ability_1_0_up_to as SelectFn), &(largest_ability_1_0_up_to as cantrip_api::select::LargestFn), TargetSpec::COUNTED, "up to X"))];
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; (t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; g.phase_out(&objects_1);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "March of Swirling Mist", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_1_TARGETS, ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_cost: Some(Cost { exile_from_hand_for_less: Some(ExileFromHandForLess { filter: ability_0_0, less_each: 2 }), ..Cost::NONE }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
