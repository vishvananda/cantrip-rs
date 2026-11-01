// Hall of the Bandit Lord
// status: planned
// requires: mana spending effect
// Hall of the Bandit Lord enters tapped.
// {T}, Pay 3 life: Add {C}. If that mana is spent on a creature spell, it gains haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
#[allow(dead_code, unused_variables)] fn mana_use_224(q: &dyn Query, asker: &Asker, spending: &Spending) -> bool { true }

pub static CARD: CardDef = CardDef { name: "Hall of the Bandit Lord", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(life(3)), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_each(&[]) }.only_for(planned_value!("mana spending effect", ManaRestriction::ABILITIES => ManaRestriction::Rule(&ManaRule { spend: mana_use_224, spent: Some(ManaSpent { on: "CardType", does: "SpentOn" }) }))), "{T}, Pay 3 life: Add {C}. If that mana is spent on a creature spell, it gains haste.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
