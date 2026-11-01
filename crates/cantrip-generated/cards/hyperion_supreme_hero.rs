// Hyperion, Supreme Hero
// status: ready
// Flash
// Flying
// If a source would deal damage to you or a Hero you control, prevent all but 1 of that damage.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && match *target { Target::Object(o) => (q.chars(o).has_subtype(q.words(asker.source).subtype(Subtype::Hero)) && Some(asker.controller) == Some(q.controller(o))), Target::Player(p) => Some(asker.controller) == Some(p), _ => false } { return true; } }
 false }
#[allow(unused_variables)] fn ability_2_amount(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> u16 { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; amount.saturating_sub((1i64).max(0) as u16) }

pub static CARD: CardDef = CardDef { name: "Hyperion, Supreme Hero", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [3, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eternal,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(7), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "If a source would deal damage to you or a Hero you control, prevent all but 1 of that damage.", applies: ability_2_applies, amount: Some(ability_2_amount) }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
