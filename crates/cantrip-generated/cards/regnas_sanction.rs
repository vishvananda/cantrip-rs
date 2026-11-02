// Regna's Sanction
// status: planned
// requires: object collection Chosen
// For each player, choose friend or foe. Each friend puts a +1/+1 counter on each creature they control. Each foe chooses one untapped creature they control, then taps the rest.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let card_0 = g.card_id(&CARD); let mut words_0: Vec<(PlayerId, u8)> = Vec::new(); for player in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } { let word = cantrip_mechanics::choose::option(g, asker.controller, asker.source, card_0, 0..2)?; words_0.push((player, word)); }
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 0)).collect::<Vec<PlayerId>>() };
let mut gathered_2: Vec<(PlayerId, ObjectId, CounterKind, i16)> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let mut counters_4: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_4.push((object, CounterKind::PlusOne, amount)); } }
gathered_2.extend(counters_4.iter().map(|c| (bound_0, c.0, c.1, c.2)));
    } 
let had_2: Vec<_> = gathered_2.iter().map(|c| g.state().obj(c.1).counters(c.2)).collect(); g.add_counters_together_by(&gathered_2); let applied_2: Vec<ObjectId> = gathered_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.1).counters(c.2) > *had).map(|(c, _)| c.1).collect();
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 1)).collect::<Vec<PlayerId>>() };
let mut gathered_6: Vec<ObjectId> = Vec::new();
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { gathered_6.extend({ let q: &dyn Query = g; { let without: Vec<ObjectId> = planned_value!("object collection Chosen", Vec::<ObjectId>::new() => cantrip_mechanics::choose::among(g, Some(bound_0).into_iter().collect::<Vec<PlayerId>>(), asker.source, q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>(), cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32))?); (q.state().battlefield().iter().copied().filter(|&id| (!(q.state().obj(id).tapped) && q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|id| !without.contains(id)).collect::<Vec<ObjectId>>() } });
    } 
let objects_8 = { let q: &dyn Query = g; (gathered_6.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_8: Vec<ObjectId> = objects_8.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_8);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Regna's Sanction", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Friend", "Foe"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
