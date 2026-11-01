// Denry Klin, Editor in Chief
// status: ready
// Denry Klin enters with your choice of a +1/+1, first strike, or vigilance counter on it.
// Whenever a nontoken creature you control enters, if Denry Klin has counters on it, put the same number of each kind of counter on that creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); cantrip_mechanics::choose::option_as_enters(g, entering, &CARD, 0..3)?; match entering.chosen_option.map(|chosen| chosen.index) {
Some(0) => { entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(1);  }
Some(1) => { entering.counters[CounterKind::FirstStrike as usize] = entering.counters[CounterKind::FirstStrike as usize].saturating_add(1);  }
Some(2) => { entering.counters[CounterKind::Vigilance as usize] = entering.counters[CounterKind::Vigilance as usize].saturating_add(1);  }
_ => {} }
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (CounterKind::ALL.iter().map(|k| i64::from(q.state().obj(candidate).counters(*k))).sum::<i64>()) > (0i64)) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (!(q.state().obj(id).token) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let sources_1: Vec<[i16; CounterKind::COUNT]> = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).iter().map(|&from| g.obj(from).counter_array()).collect(); let recipients_1: Vec<ObjectId> = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let puts_1: Vec<(ObjectId, CounterKind, i16)> = recipients_1.iter().flat_map(|&to| sources_1.iter().flat_map(move |counts| CounterKind::ALL.into_iter().map(move |kind| (to, kind, counts[kind as usize])))).collect(); g.add_counters_together(&puts_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Denry Klin, Editor in Chief", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Whenever a nontoken creature you control enters, if Denry Klin has counters on it, put the same number of each kind of counter on that creature.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, labels: &["PowerToughness", "Named", "Named"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
