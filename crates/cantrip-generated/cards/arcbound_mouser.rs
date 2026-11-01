// Arcbound Mouser
// status: ready
// Lifelink
// Modular 1 (This creature enters with a +1/+1 counter on it. When it dies, you may put its +1/+1 counters on target artifact creature.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_modular(_: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let kind = CounterKind::PlusOne as usize; entering.counters[kind] = entering.counters[kind].saturating_add(1); Ok(()) }

pub static CARD: CardDef = CardDef { name: "Arcbound Mouser", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::modular::TRIGGER], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_modular), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
