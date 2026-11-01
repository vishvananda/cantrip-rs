// Spirit of Resilience
// status: planned
// requires: object collection Scoped
// Whenever one or more cards leave your graveyard, put a +1/+1 counter on this creature, then you may have this creature become a copy of an artifact or creature card from among those cards until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if from.is(ZoneKind::Graveyard) && !q.obj(*old).token && !q.obj(*old).card_copy && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
{
planned!("object-collection-scoped", ChooseObjects, "constraints" = "[]", "count" = "{\"AsManyAsCan\":{\"args\":1,\"kind\":\"Constant\"}}", "objects" = "{\"args\":{\"filter\":{\"args\":[{\"args\":[{\"args\":\"Artifact\",\"kind\":\"CardType\"},{\"args\":\"Creature\",\"kind\":\"CardType\"}],\"kind\":\"Either\"},{\"args\":{\"kind\":\"Scoped\"},\"kind\":\"InSet\"}],\"kind\":\"All\"},\"kind\":\"Card\",\"scope\":{\"Players\":{\"players\":{\"kind\":\"All\"},\"zone\":\"Graveyard\"}}},\"kind\":\"Zone\"}", "player" = { let q: &dyn Query = g; Some(asker.controller) });
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let original_2 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1)).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let values_2 = { let q: &dyn Query = g; original_2.first().copied().map(|original: ObjectId| q.copiable_values(original)) }; if let (Some(source), Some(values)) = (source_2, values_2) { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.become_copy(source, object, values, duration_2); } }); }
g.end_sentence();
hoisted_3.extend(objects_2.clone());

}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Spirit of Resilience", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever one or more cards leave your graveyard, put a +1/+1 counter on this creature, then you may have this creature become a copy of an artifact or creature card from among those cards until end of turn.").targets(&[]).once_per_batch()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
