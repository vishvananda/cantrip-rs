// The Tenth Doctor
// status: planned
// requires: engine: no Time Travel
// requires: exiled card gains suspend
// Allons-y! — Whenever you attack, exile cards from the top of your library until you exile a nonland card. Put three time counters on it. If it doesn't have suspend, it gains suspend.
// Timey-Wimey — {7}: Time travel three times. Activate only as a sorcery. (For each suspended card you own and each permanent you control with a time counter on it, you may add or remove a time counter. Then do it two more times.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::AttackersDeclared { player, count } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if *count > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let owner_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let (found_0, rest_0) = cantrip_mechanics::library::reveal_until_n(g, owner_0, count_0, &|q: &dyn Query, id: ObjectId| !(q.chars(id).is(CardType::Land))); let shown_0 = found_0.len() + rest_0.len(); let objects_0: Vec<ObjectId> = g.state().zone(Zone::library(owner_0)).iter().copied().take(shown_0).collect();
let objects_1 = { let q: &dyn Query = g; objects_0.clone() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (3i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (((moved_1.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::Time, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let cards_3 = { let q: &dyn Query = g; ((moved_1.clone()).into_iter().filter(|&id| !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() };
planned!("exiled-card-gains-suspend" => for card in cards_3 { cantrip_mechanics::suspend::gains_suspend(g, card); });
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for _ in 0..({ let q: &dyn Query = g; (3i64).max(0) as u32 }) {
planned!("engine-no-time-travel" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; cantrip_mechanics::time_travel::time_travel(g, player, asker.source)?; });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Tenth Doctor", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::TimeLord,Subtype::Doctor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKERS_DECLARED.0), ability_0_event), ability_0, "Allons-y! — Whenever you attack, exile cards from the top of your library until you exile a nonland card. Put three time counters on it. If it doesn't have suspend, it gains suspend.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "Timey-Wimey — {7}: Time travel three times. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
