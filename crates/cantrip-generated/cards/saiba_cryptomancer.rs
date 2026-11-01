// Saiba Cryptomancer
// status: ready
// Flash
// Backup 1 (When this creature enters, put a +1/+1 counter on target creature. If that's another creature, it gains the following ability until end of turn.)
// Hexproof
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_backup(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { cantrip_mechanics::backup::resolve(g, item, 1, &[Keyword::Hexproof], None) }

pub static CARD: CardDef = CardDef { name: "Saiba Cryptomancer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Moonfolk,Subtype::Ninja]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Hexproof]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::backup::trigger("Backup 1", ability_1_backup)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
