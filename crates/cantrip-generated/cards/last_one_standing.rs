// Last One Standing
// status: ready
// Choose a creature at random, then destroy the rest.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_0: Vec<ObjectId> = Vec::new(); while picked_0.len() < count_0 && !pool_0.is_empty() { let at = g.choose_at_random(pool_0.len()); picked_0.push(pool_0.remove(at)); }
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!((picked_0.clone()).get(0 as usize).copied() == Some(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Last One Standing", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
