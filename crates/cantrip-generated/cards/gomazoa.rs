// Gomazoa
// status: ready
// Defender, flying
// {T}: Put this creature and each creature it's blocking on top of their owners' libraries, then those players shuffle.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (Some(asker.source) == Some(id) || (q.chars(id).is(CardType::Creature) && { let blocker = Some(asker.source); q.state().blocks.iter().any(|b| b.attacker == id && Some(b.blocker) == blocker) }))).collect::<Vec<ObjectId>>() }; let moved_0 = g.put_into_library(&objects_0, LibraryPlace::ShuffledIn);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gomazoa", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Jellyfish]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Defender,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{T}: Put this creature and each creature it's blocking on top of their owners' libraries, then those players shuffle.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
