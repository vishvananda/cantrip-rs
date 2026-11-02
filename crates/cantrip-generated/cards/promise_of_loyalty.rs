// Promise of Loyalty
// status: planned
// requires: continuous rule Prohibit Attack
// Each player puts a vow counter on a creature they control and sacrifices the rest. Each of those creatures can't attack you or planeswalkers you control for as long as it has a vow counter on it.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(PlayerId, ObjectId, CounterKind, i16)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut counters_3: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
{ let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; let player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let chosen = cantrip_mechanics::choose::select(g, player, asker.source, Purpose::PutCounters(CounterKind::Vow, amount.clamp(0, 255) as u8), candidates, cantrip_mechanics::select::Selection::of(count, &[]))?; for object in chosen { counters_3.push((object, CounterKind::Vow, amount)); } }
gathered_1.extend(counters_3.iter().map(|c| (bound_0, c.0, c.1, c.2)));
    } 
let had_1: Vec<_> = gathered_1.iter().map(|c| g.state().obj(c.1).counters(c.2)).collect(); g.add_counters_together_by(&gathered_1); let applied_1: Vec<ObjectId> = gathered_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.1).counters(c.2) > *had).map(|(c, _)| c.1).collect();
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && !((applied_1.clone()).contains(&id)))).collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_4 { let controller = g.controller(object); if players_4.contains(&controller) && g.can_sacrifice(controller, object) { picks_4.push((controller, object)); } }
let sacrificed_4 = g.sacrifice_each(&picks_4);
g.end_sentence();
let objects_5 = { let q: &dyn Query = g; (applied_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::WhileCounter(CounterKind::Vow);
let kind_5_0_0 = { let q: &dyn Query = g; planned_value!("continuous rule Prohibit Attack", EffectKind::CantBlock => EffectKind::ObjectRule { card: { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, index: 0 }) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("continuous-rule-prohibit-attack" => lasting_object_rules: LastingObjectRule { text: "Each player puts a vow counter on a creature they control and sacrifices the rest. Each of those creatures can't attack you or planeswalkers you control for as long as it has a vow counter on it.", rule: Rule::Cant(ObjectAction::Attack(|q: &dyn Query, asker: &Asker, defender: Target| (match defender { Target::Player(p) => (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&p), Target::Object(id) => true && q.chars(id).is(CardType::Planeswalker) && (Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).contains(&q.controller(id)), _ => false }))) });

pub static CARD: CardDef = CardDef { name: "Promise of Loyalty", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
