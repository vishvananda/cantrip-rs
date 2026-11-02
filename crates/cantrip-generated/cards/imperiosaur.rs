// Imperiosaur
// status: planned
// requires: engine: no rule on the mana spent on a spell
// Spend only mana produced by basic lands to cast this spell.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("engine-no-rule-on-the-mana-spent-on-a-spell" => #[allow(unused_variables)] fn ability_0_mana_source_0(q: &dyn Query, asker: &Asker, source: ObjectId) -> bool { (q.chars(source).supertypes.basic && q.chars(source).is(CardType::Land)) });
planned_item!("engine-no-rule-on-the-mana-spent-on-a-spell" => mana_spent: Some((Spent::Whole, ManaSpendRule::ProducedBy(ability_0_mana_source_0))));

pub static CARD: CardDef = CardDef { name: "Imperiosaur", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dinosaur]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
