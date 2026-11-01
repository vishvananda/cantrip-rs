// Elite Inquisitor
// status: ready
// First strike, vigilance
// Protection from Vampires, from Werewolves, and from Zombies
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "Elite Inquisitor", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::FirstStrike,Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Vampire) }>("Protection from Vampire"),cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Werewolf) }>("Protection from Werewolf"),cantrip_mechanics::protection::from_subtype::<{ cantrip_mechanics::lands::sub(Subtype::Zombie) }>("Protection from Zombie")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
