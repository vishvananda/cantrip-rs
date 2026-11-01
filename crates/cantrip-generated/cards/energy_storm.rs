// Energy Storm
// status: ready
// Cumulative upkeep {1} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)
// Prevent all damage that would be dealt by instant and sorcery spells.
// Creatures with flying don't untap during their controllers' untap steps.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Instant) || q.chars(source).is(CardType::Sorcery)) && true { return true; } }
 false }
#[allow(unused_variables)] fn ability_2_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && (*so_far).keywords.has(Keyword::Flying)) }
#[allow(unused_variables)] fn ability_2_0_rule(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).is(CardType::Creature) && q.chars(id).keywords.has(Keyword::Flying)) }

pub static CARD: CardDef = CardDef { name: "Energy Storm", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::cumulative_upkeep!(cantrip_mechanics::cumulative_upkeep::Payment::Mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), "Cumulative upkeep")], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "Prevent all damage that would be dealt by instant and sorcery spells.", applies: ability_1_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[ability_2_0_rule], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
