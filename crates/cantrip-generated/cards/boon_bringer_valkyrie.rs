// Boon-Bringer Valkyrie
// status: ready
// Backup 1 (When this creature enters, put a +1/+1 counter on target creature. If that's another creature, it gains the following abilities until end of turn.)
// Flying, first strike, lifelink
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_backup(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { cantrip_mechanics::backup::resolve(g, item, 1, &[Keyword::Flying,Keyword::FirstStrike,Keyword::Lifelink], None) }

pub static CARD: CardDef = CardDef { name: "Boon-Bringer Valkyrie", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::FirstStrike,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::backup::trigger("Backup 1", ability_0_backup)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
