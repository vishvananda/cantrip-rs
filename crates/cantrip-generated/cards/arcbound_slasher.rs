// Arcbound Slasher
// status: ready
// Modular 4 (This creature enters with four +1/+1 counters on it. When it dies, you may put its +1/+1 counters on target artifact creature.)
// Riot (This creature enters with your choice of an additional +1/+1 counter or haste.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_modular(_: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let kind = CounterKind::PlusOne as usize; entering.counters[kind] = entering.counters[kind].saturating_add(4); Ok(()) }
fn ability_1_riot(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { cantrip_mechanics::riot::enters(g, entering, &CARD) }
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_0_modular(g, entering)?; ability_1_riot(g, entering)?;  Ok(()) }

pub static CARD: CardDef = CardDef { name: "Arcbound Slasher", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::modular::TRIGGER], statics: &[cantrip_mechanics::riot::HASTE_STATIC], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(entering_all), modal: None, enchant: None, labels: cantrip_mechanics::riot::LABELS, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
