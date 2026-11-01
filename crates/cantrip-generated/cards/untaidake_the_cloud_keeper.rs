// Untaidake, the Cloud Keeper
// status: planned
// requires: mana spending predicate
// Untaidake enters tapped.
// {T}, Pay 2 life: Add {C}{C}. Spend this mana only to cast legendary spells.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(dead_code, unused_variables)] fn mana_use_224(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { spending.purpose == SpendingFor::Casting && spending.source.is_some_and(|spell| q.chars(spell).supertypes.legendary) }

pub static CARD: CardDef = CardDef { name: "Untaidake, the Cloud Keeper", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(life(2)), Adds { pool: ManaPool([0, 0, 0, 0, 0, 2]), ..add_each(&[]) }.only_for(planned_value!("mana spending predicate", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_224, spent: None }))), "{T}, Pay 2 life: Add {C}{C}. Spend this mana only to cast legendary spells.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
