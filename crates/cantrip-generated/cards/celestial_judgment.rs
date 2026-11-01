// Celestial Judgment
// status: ready
// For each different power among creatures on the battlefield, choose a creature with that power. Destroy each creature not chosen this way.
#[allow(unused_imports)] use cantrip_api::*;
fn different_powers_key(q: &dyn Query, id: ObjectId) -> i16 { q.chars(id).power.unwrap_or(0) }
fn different_powers(q: &dyn Query, choice: &Choice, _: u32) -> Verdict { let used: Vec<i16> = choice.selected.iter().map(|&c| different_powers_key(q, c)).collect(); let apart = used.iter().enumerate().all(|(i, k)| !used[..i].contains(k)); Verdict { complete: apart, addable: Bits::of(choice.candidates.len(), |i| !used.contains(&different_powers_key(q, choice.candidates[i]))) } }
fn largest_different_powers(q: &dyn Query, choice: &Choice, _: u32) -> Vec<ObjectId> { cantrip_api::select::largest_distinct(choice, |c| different_powers_key(q, c)) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let pool_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan(((q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32) }; let picked_0 = cantrip_mechanics::choose::select(g, player_0, asker.source, Purpose::Keep, pool_0, cantrip_mechanics::select::Selection::of(count_0, &[]).with(cantrip_api::select::set_part(&(different_powers as SelectFn), &(largest_different_powers as cantrip_api::select::LargestFn), 0, "with different powers")))?;
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; { let chosen: Vec<ObjectId> = picked_0.clone(); (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|o| !chosen.contains(o)).collect::<Vec<ObjectId>>() } }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Celestial Judgment", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
