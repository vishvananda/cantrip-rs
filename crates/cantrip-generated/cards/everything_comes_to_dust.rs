// Everything Comes to Dust
// status: ready
// Convoke (Your creatures can help cast this spell. Each creature you tap while casting this spell pays for {1} or one mana of that creature's color.)
// Exile all creatures except those that share a creature type with a creature that convoked this spell, all artifacts, and all enchantments.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Creature) && !((q.state().battlefield().iter().copied().filter(|&id| Some(asker.source).is_some_and(|o: ObjectId| q.history().convoked(o).any(|c| c == id))).collect::<Vec<ObjectId>>()).iter().any(|&o| { let (a, b) = (&q.chars(id), &q.chars(o)); a.subtypes.iter().any(|s| SubtypeSet::CREATURE_TYPES.has(s) && b.subtypes.has(s)) }))) || q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Enchantment))).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Everything Comes to Dust", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, pay_with: PayWith { convoke: true, ..PayWith::NONE }, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
