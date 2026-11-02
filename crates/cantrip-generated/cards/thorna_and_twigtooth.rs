// Thorna and Twigtooth
// status: planned
// requires: engine: no perpetual effects
// Thorna and Twigtooth enters with two -1/-1 counters on it.
// Whenever Thorna and Twigtooth attacks, remove all counters from target creature you control. Each opponent loses X life, you gain X life, and the topmost creature card in your library perpetually gets +X/+X, where X is the number of counters removed this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (2i64).max(0) as i16 }; entering.counters[CounterKind::MinusOne as usize] = entering.counters[CounterKind::MinusOne as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_1_target_0)];
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut removed_1: i64 = 0;
let objects_1_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
cantrip_api::instruction(g, |g| { for object in objects_1_0 { removed_1 += CounterKind::ALL.iter().map(|k| i64::from(g.state().obj(object).counters(*k))).sum::<i64>(); cantrip_mechanics::effects::remove_all_counters(g, object); } });
g.end_sentence();
let value_2: i64 = { let q: &dyn Query = g; (removed_1 as i64) };
g.end_sentence();
let amount_3 = { let q: &dyn Query = g; (value_2).clamp(0, 2147483647) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
for player in players_3 { g.lose_life(player, amount_3); }
g.end_sentence();
let amount_4 = { let q: &dyn Query = g; (value_2).clamp(0, 2147483647) as i32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.gain_life(player, amount_4); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_6: Vec<ObjectId> = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() };
for &object in &objects_6 { let kind_6_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((value_2).clamp(-32768, 32767) as i16, (value_2).clamp(-32768, 32767) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_6_0); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Thorna and Twigtooth", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Giant,Subtype::Treefolk]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(7), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever Thorna and Twigtooth attacks, remove all counters from target creature you control. Each opponent loses X life, you gain X life, and the topmost creature card in your library perpetually gets +X/+X, where X is the number of counters removed this way.").targets(ABILITY_1_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
