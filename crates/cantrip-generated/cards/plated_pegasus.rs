// Plated Pegasus
// status: ready
// Flash (You may cast this spell any time you could cast an instant.)
// Flying
// If a spell would deal damage to a permanent or player, prevent 1 damage that spell would deal to that permanent or player.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if q.state().obj(source).zone.is(ZoneKind::Stack) && true && match *target { Target::Object(o) => true, Target::Player(p) => true, _ => false } { return true; } }
 false }
#[allow(unused_variables)] fn ability_2_amount(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> u16 { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; (1i64).max(0) as u16 }

pub static CARD: CardDef = CardDef { name: "Plated Pegasus", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Pegasus]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "If a spell would deal damage to a permanent or player, prevent 1 damage that spell would deal to that permanent or player.", applies: ability_2_applies, amount: Some(ability_2_amount) }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
