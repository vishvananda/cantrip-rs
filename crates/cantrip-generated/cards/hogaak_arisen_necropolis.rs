// Hogaak, Arisen Necropolis
// status: planned
// requires: engine: no rule on the mana spent on a spell
// You can't spend mana to cast this spell.
// Convoke, delve (Each creature you tap while casting this spell pays for {1} or one mana of that creature's color. Each card you exile from your graveyard pays for {1}.)
// You may cast this card from your graveyard.
// Trample
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-rule-on-the-mana-spent-on-a-spell" => mana_spent: Some((Spent::Whole, ManaSpendRule::Nothing)));

pub static CARD: CardDef = CardDef { name: "Hogaak, Arisen Necropolis", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::B, Mana::G)),Some((Mana::B, Mana::G)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Avatar]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(8), toughness: Some(8), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::CastFromYourGraveyard,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, pay_with: PayWith { convoke: true, delve: true, ..PayWith::NONE }, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
