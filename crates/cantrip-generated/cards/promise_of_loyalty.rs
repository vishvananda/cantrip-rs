// Promise of Loyalty
// status: planned
// requires: continuous rule Prohibit Attack
// requires: counter kind Vow
// requires: per-player PutCounters
// Each player puts a vow counter on a creature they control and sacrifices the rest. Each of those creatures can't attack you or planeswalkers you control for as long as it has a vow counter on it.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
planned!("per-player-put-counters" => g.begin_batch(););
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
{ let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let chosen = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::PutCounters(planned_value!("counter kind Vow", CounterKind::Charge => CounterKind::Vow), amount.clamp(0, 255) as u8), candidates, cantrip_mechanics::select::Selection::of(count, &[]))?; for object in chosen { counters_1.push((object, planned_value!("counter kind Vow", CounterKind::Charge => CounterKind::Vow), amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
 each_2[index_0] = applied_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
planned!("per-player-put-counters" => g.end_batch(););
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !((all_2.clone()).contains(&id)))).collect::<Vec<ObjectId>>() }; let players_3 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_3: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_3 { let controller = g.controller(object); if players_3.contains(&controller) && g.can_sacrifice(controller, object) { picks_3.push((controller, object)); } }
let sacrificed_3 = g.sacrifice_each(&picks_3);
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (all_2.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::WhileCounter(planned_value!("counter kind Vow", CounterKind::Charge => CounterKind::Vow));
let kind_4_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule Prohibit Attack", EffectKind::CantBlock => EffectKind::Rule(Rule::Cant(Action::Attack(|q: &dyn Query, asker: &Asker, defender: Target| (match defender { Target::Player(p) => (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false }))))) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Promise of Loyalty", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
