// Pillar of the Paruns
// status: planned
// requires: mana spending predicate
// {T}: Add one mana of any color. Spend this mana only to cast a multicolored spell.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(dead_code, unused_variables)] fn mana_use_0(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| (q.chars(spell).colors.iter().count() as i64) > (1i64)) }

pub static CARD: CardDef = CardDef { name: "Pillar of the Paruns", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_any_color(1).only_for(planned_value!("mana spending predicate", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_0, spent: None }))), "{T}: Add one mana of any color. Spend this mana only to cast a multicolored spell.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
