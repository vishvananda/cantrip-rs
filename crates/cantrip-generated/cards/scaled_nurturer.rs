// Scaled Nurturer
// status: planned
// requires: mana spending effect
// {T}: Add {G}. When you spend this mana to cast a Dragon creature spell, you gain 2 life.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(dead_code, unused_variables)] fn mana_use_0(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { true }

pub static CARD: CardDef = CardDef { name: "Scaled Nurturer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon,Subtype::Druid]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 1, 0]), ..add_each(&[]) }.only_for(planned_value!("mana spending effect", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_0, spent: Some(ManaSpent { on: "All", does: "WhenSpent" }) }))), "{T}: Add {G}. When you spend this mana to cast a Dragon creature spell, you gain 2 life.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
