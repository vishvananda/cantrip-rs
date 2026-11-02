// Bloodspore Thrinax
// status: planned
// requires: engine: no Devour
// Devour 1 (As this creature enters, you may sacrifice any number of creatures. It enters with that many +1/+1 counters on it.)
// Each other creature you control enters with an additional X +1/+1 counters on it, where X is the number of +1/+1 counters on this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_applies(q: &dyn Query, asker: &Asker, proposal: &Proposal) -> bool { if let Proposal::ZoneChange(z) = proposal { let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if z.to.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(z.controller))) { return true; } }
 false }
#[allow(unused_variables, unused_mut)] fn ability_1_apply(q: &dyn Query, asker: &Asker, proposal: &mut Proposal) { let seen = *proposal; if let Proposal::ZoneChange(seen) = &seen { let z = seen; let id = z.object; let subject = Target::Object(id); let data = [u32::MAX; 2]; if let Proposal::ZoneChange(z) = proposal { z.counters[CounterKind::PlusOne as usize] = z.counters[CounterKind::PlusOne as usize].saturating_add(((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::PlusOne))).sum::<i64>()).min(i16::MAX.into()) as i16); } } }
planned_item!("engine-no-devour" => enters: cantrip_mechanics::devour::enters::<1>);

pub static CARD: CardDef = CardDef { name: "Bloodspore Thrinax", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[replacement(ReplacementKind::ZoneChange, ability_1_applies, ability_1_apply, "Each other creature you control enters with an additional X +1/+1 counters on it, where X is the number of +1/+1 counters on this creature.")], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
