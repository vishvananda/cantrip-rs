// Infernal Darkness
// status: planned
// requires: cumulative upkeep paid other than in mana or life
// Cumulative upkeep—Pay {B} and 1 life. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// If a land is tapped for mana, it produces {B} instead of any other type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ManaProduction(m) = proposal { let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if q.chars(id).is(CardType::Land) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ManaProduction(seen) = &seen { let m = seen; let id = m.permanent; let amount = m.total(); let subject = Target::Object(id); let data = [amount, u32::MAX]; if let Proposal::ManaProduction(m) = proposal { let total = m.total(); m.mana = ManaPool::default(); m.mana.0[Mana::B as usize] = total; } } }
planned_item!("cumulative-upkeep-paid-other-than-in-mana-or-life" => abilities: cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Cost(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(1))), "Cumulative upkeep"));

pub static CARD: CardDef = CardDef { name: "Infernal Darkness", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::ManaProduction, ability_1_applies, ability_1_apply, "If a land is tapped for mana, it produces {B} instead of any other type.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
