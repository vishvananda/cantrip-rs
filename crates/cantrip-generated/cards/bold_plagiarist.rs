// Bold Plagiarist
// status: planned
// requires: counters the event put
// Flash
// Whenever an opponent puts one or more counters on a creature they control, they put the same number and kind of counters on this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::CountersPut { object, kind, amount, by, .. } = event { let id = *object; let subject = Target::Object(id); let data = [*amount as u32, id.0]; if { let data = [0u32, u32::from(by.0)]; (q.chars(id).is(CardType::Creature) && Some(PlayerId(data[1] as u8)) == Some(q.controller(id))) } && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*by) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("counters-the-event-put" => let to_0: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let put_0: Vec<(CounterKind, i16)> = g.counters_put_by(item); let puts: Vec<(ObjectId, CounterKind, i16)> = to_0.iter().flat_map(|&o| put_0.iter().map(move |&(kind, amount)| (o, kind, amount))).collect(); g.add_counters_together(&puts););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bold Plagiarist", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::COUNTERS.0), ability_1_event), ability_1, "Whenever an opponent puts one or more counters on a creature they control, they put the same number and kind of counters on this creature.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
