// Case of the Locked Hothouse
// status: ready
// You may play an additional land on each of your turns.
// To solve — You control seven or more lands. (If unsolved, solve at the beginning of your end step.)
// Solved — You may look at the top card of your library any time, and you may play lands and cast creature and enchantment spells from the top of your library.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_to_solve(q: &dyn Query, asker: &Asker) -> bool { ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64) >= (7i64) }
#[allow(unused_variables)] fn ability_1_solved_0_0_0_while(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }
#[allow(unused_variables)] fn ability_1_solved_0_0_1_while(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }
#[allow(unused_variables)] fn ability_1_solved_0_0_2_rule(q: &dyn Query, asker: &Asker, spell: ObjectId) -> bool { (q.chars(spell).is(CardType::Creature) || q.chars(spell).is(CardType::Enchantment)) }
#[allow(unused_variables)] fn ability_1_solved_0_0_2_while(q: &dyn Query, asker: &Asker) -> bool { q.is_solved(asker.source) }

pub static CARD: CardDef = CardDef { name: "Case of the Locked Hothouse", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Case]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::solved::to_solve(ability_1_to_solve, "To solve (If unsolved, solve at the beginning of your end step.)")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::ExtraLandPlays(1),PlayRule::While(ability_1_solved_0_0_0_while, &[PlayRule::LookAtLibraryTop]),PlayRule::While(ability_1_solved_0_0_1_while, &[PlayRule::LibraryTopLands]),PlayRule::While(ability_1_solved_0_0_2_while, &[PlayRule::LibraryTopSpells(ability_1_solved_0_0_2_rule, None)])], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
