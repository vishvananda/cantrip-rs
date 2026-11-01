// Rampaging Rendhorn
// status: ready
// Riot (This creature enters with your choice of a +1/+1 counter or haste.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_riot(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { cantrip_mechanics::riot::enters(g, entering, &CARD) }

pub static CARD: CardDef = CardDef { name: "Rampaging Rendhorn", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::riot::HASTE_STATIC], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_riot), modal: None, enchant: None, labels: cantrip_mechanics::riot::LABELS, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
