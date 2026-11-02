// Mana Maze
// status: ready
// Players can't cast spells that share a color with the spell most recently cast this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, asker: &Asker, player: PlayerId, spell: ObjectId) -> bool { !(true && (q.history().this_turn().iter().rev().find_map(|r| match r.event { Event::Cast(cast) => Some(cast.spell), _ => None }).into_iter().collect::<Vec<ObjectId>>()).first().copied().is_some_and(|o: ObjectId| { let (a, b) = (&q.chars(spell), &q.history().cast_of(o).filter(|_| q.state().obj(o).became.is_some()).map_or_else(|| q.chars(o), |cast| cast.chars(q))); a.colors.intersects(b.colors) })) }

pub static CARD: CardDef = CardDef { name: "Mana Maze", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, spell_rules: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
