// Captain Marvel, Apex Avenger
// status: planned
// requires: counters the event put
// Flying, double strike, indestructible
// Whenever you put one or more counters on another creature, if it's not a Kree, you may put the same number and kind of counters on Captain Marvel.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersPut { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) && Some(asker.controller) == Some(*by) && subject.object().is_some_and(|candidate: ObjectId| !(q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Kree)))) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| !(q.chars(candidate).has_subtype(q.words(asker.source).subtype(Subtype::Kree)))) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
planned!("counters-the-event-put" => let to_1: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let put_1: Vec<(CounterKind, i16)> = g.counters_put_by(item); let puts: Vec<(ObjectId, CounterKind, i16)> = to_1.iter().flat_map(|&o| put_1.iter().map(move |&(kind, amount)| (o, kind, amount))).collect(); g.add_counters_together(&puts););
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Captain Marvel, Apex Avenger", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Kree,Subtype::Hero]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::DoubleStrike,Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::COUNTERS.0), ability_3_event), ability_3, "Whenever you put one or more counters on another creature, if it's not a Kree, you may put the same number and kind of counters on Captain Marvel.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
