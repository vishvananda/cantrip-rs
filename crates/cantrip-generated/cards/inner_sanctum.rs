// Inner Sanctum
// status: ready
// Cumulative upkeep—Pay 2 life. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// Prevent all damage that would be dealt to creatures you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if (q.chars(o).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(o)))) { return true; } }
 false }

pub static CARD: CardDef = CardDef { name: "Inner Sanctum", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Life(2), "Cumulative upkeep")], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all damage that would be dealt to creatures you control", applies: ability_1_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
