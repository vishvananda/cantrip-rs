// Sovereign Okinec Ahau
// status: planned
// requires: base power
// requires: object state PowerAboveBase
// Ward {2}
// Whenever Sovereign Okinec Ahau attacks, for each creature you control with power greater than that creature's base power, put a number of +1/+1 counters on that creature equal to the difference.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)) && planned_value!("object state PowerAboveBase", false => q.chars(id).power.map(i64::from) > Some(q.base_power(id))))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for &object in &objects_0 { let amount = { let q: &dyn Query = g; ((Some(object).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0))) - Some(object).map_or(0i64, |o: ObjectId| planned_value!("base power", 0i64 => q.base_power(o))))).max(0) as i16 }; counters_0.push((object, CounterKind::PlusOne, amount)); } g.add_counters_together(&counters_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Sovereign Okinec Ahau", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}"),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Sovereign Okinec Ahau attacks, for each creature you control with power greater than that creature's base power, put a number of +1/+1 counters on that creature equal to the difference.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
