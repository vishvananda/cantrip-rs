// All Suns' Dawn
// status: ready
// For each color, return up to one target card of that color from your graveyard to your hand. Exile All Suns' Dawn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn one_for_each_0_bits(q: &dyn Query, asker: &Asker, id: ObjectId) -> u32 { let mut bits = 0u32; if q.chars(id).colors.has(q.words(asker.source).color(Color::White)) { bits |= 1 << 0; } if q.chars(id).colors.has(q.words(asker.source).color(Color::Blue)) { bits |= 1 << 1; } if q.chars(id).colors.has(q.words(asker.source).color(Color::Black)) { bits |= 1 << 2; } if q.chars(id).colors.has(q.words(asker.source).color(Color::Red)) { bits |= 1 << 3; } if q.chars(id).colors.has(q.words(asker.source).color(Color::Green)) { bits |= 1 << 4; } bits }
fn one_for_each_0_fits(q: &dyn Query, choice: &Choice, set: &[ObjectId]) -> bool { let asker = &Asker::new(choice.source, choice.chooser); let mut used = vec![0u32]; for &id in set { let bits = one_for_each_0_bits(q, asker, id); let mut next: Vec<u32> = Vec::new(); for &taken in &used { for each in 0..5 { let one = 1u32 << each; if bits & one != 0 && taken & one == 0 && !next.contains(&(taken | one)) { next.push(taken | one); } } } used = next; } !used.is_empty() }
fn one_for_each_0(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { Verdict { complete: one_for_each_0_fits(q, choice, choice.selected), addable: Bits::of(choice.candidates.len(), |i| { let mut set = choice.selected.to_vec(); set.push(choice.candidates[i]); one_for_each_0_fits(q, choice, &set) }) } }
fn largest_one_for_each_0(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { let mut kept: Vec<ObjectId> = Vec::new(); for &id in choice.candidates { kept.push(id); if !one_for_each_0_fits(q, choice, &kept) { kept.pop(); } } kept }
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 5).together(cantrip_api::select::set_part(&(one_for_each_0 as SelectFn), &(largest_one_for_each_0 as cantrip_api::select::LargestFn), 0, "one for each"))];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "All Suns' Dawn", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
