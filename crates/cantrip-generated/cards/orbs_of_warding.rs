// Orbs of Warding
// status: ready
// You have hexproof. (You can't be the target of spells or abilities your opponents control.)
// If a creature would deal damage to you, prevent 1 of that damage.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn LayerQuery, source: ObjectId, player: PlayerId) -> bool { let asker = &Asker::new(source, q.controller(source)); player == asker.controller }
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && q.chars(source).is(CardType::Creature) && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return true; } }
 false }
#[allow(unused_variables)] fn ability_1_amount(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> u16 { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; (1i64).max(0) as u16 }

pub static CARD: CardDef = CardDef { name: "Orbs of Warding", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "If a creature would deal damage to you, prevent 1 of that damage.", applies: ability_1_applies, amount: Some(ability_1_amount) }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, player_keywords: &[PlayerKeywordRule { keyword: Keyword::Hexproof, applies: ability_0_0_rule }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
