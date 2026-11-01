// Thieving Varmint
// status: planned
// requires: mana spending predicate
// Deathtouch, lifelink
// {T}, Pay 1 life: Add two mana of any one color. Spend this mana only to cast spells you don't own.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(dead_code, unused_variables)] fn mana_use_0(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.state().obj(spell).owner)) }

pub static CARD: CardDef = CardDef { name: "Thieving Varmint", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Varmint]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(life(1)), add_any_color(2).only_for(planned_value!("mana spending predicate", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_0, spent: None }))), "{T}, Pay 1 life: Add two mana of any one color. Spend this mana only to cast spells you don't own.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
