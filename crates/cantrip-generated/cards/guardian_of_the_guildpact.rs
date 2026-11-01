// Guardian of the Guildpact
// status: planned
// requires: engine: no protection from this quality
// Protection from monocolored
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-protection-from-this-quality" => statics: cantrip_mechanics::protection::from_matching(ObjectKind::Any, |q, asker, source| (q.chars(source).colors.iter().count() as i64) == (1i64), "Protection from a quality"));

pub static CARD: CardDef = CardDef { name: "Guardian of the Guildpact", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
