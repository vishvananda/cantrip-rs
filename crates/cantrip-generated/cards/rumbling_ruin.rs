// Rumbling Ruin
// status: planned
// requires: a lasting rule reading what its ability chose
// When this creature enters, count the number of +1/+1 counters on creatures you control. Creatures your opponents control with power less than or equal to that number can't block this turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let value_0: i64 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::PlusOne))).sum::<i64>() };
g.end_sentence();
planned!("a-lasting-rule-reading-what-its-ability-chose", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":[{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"OpponentsOf\"},\"kind\":\"ControlledBy\"}],\"kind\":\"All\"},{\"args\":{\"comparison\":\"LessOrEqual\",\"property\":\"Power\",\"value\":{\"args\":0,\"kind\":\"BoundValue\"}},\"kind\":\"Property\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}", "changes" = "[{\"args\":{\"Prohibit\":{\"Block\":{\"kind\":\"Any\"}}},\"kind\":\"Rule\"}]", "duration" = "\"EndOfTurn\"", "evaluation" = "\"LiveObjects\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Rumbling Ruin", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, count the number of +1/+1 counters on creatures you control. Creatures your opponents control with power less than or equal to that number can't block this turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
