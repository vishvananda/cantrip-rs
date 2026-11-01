// Stumpsquall Hydra
// status: planned
// requires: engine: no commander
// When this creature enters, distribute X +1/+1 counters among it and any number of commanders.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (Some(asker.source) == Some(id) || planned_value!("engine: no commander", false => q.is_commander(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_0: u32 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.state().obj(o).x))).max(0) as u32 }; let mut puts_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new(); for (i, &object) in objects_0.iter().enumerate() { let after = (objects_0.len() - i - 1) as u32; let n = if after == 0 { left_0 } else { let most = left_0.saturating_sub(after * 0); cantrip_mechanics::choose::number(g, asker.controller, asker.source, 0.min(most), most, "Distribute")? }; left_0 -= n.min(left_0); if n > 0 { puts_0.push((object, CounterKind::PlusOne, n as i16)); } } g.add_counters_together(&puts_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stumpsquall Hydra", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hydra]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, distribute X +1/+1 counters among it and any number of commanders.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
