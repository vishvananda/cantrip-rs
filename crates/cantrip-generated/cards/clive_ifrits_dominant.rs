// Clive, Ifrit's Dominant // Ifrit, Warden of Inferno
// status: ready
// (As this Saga enters and after your draw step, add a lore counter.)
// I — Lunge — Ifrit fights up to one other target creature.
// II, III — Brimstone — Add {R}{R}{R}{R}. If Ifrit has three or more lore counters on it, exile it, then return it to the battlefield (front face up).
// When Clive enters, you may discard your hand, then draw cards equal to your devotion to red. (Each {R} in the mana costs of permanents you control counts toward your devotion to red.)
// {4}{R}{R}, {T}: Exile Clive, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(g.state().zone(Zone::hand(player)).to_vec()); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
let amount_2 = { let q: &dyn Query = g; (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(cantrip_mechanics::devotion::devotion(q, p, Color::Red)))) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_2 { g.draw(player, amount_2); }
g.end_sentence();
hoisted_3.extend(discarded_1.clone());
hoisted_4.extend({ let q: &dyn Query = g; discarded_1.clone() });
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_5 = g.exile_all(&objects_5);
g.end_sentence();
let objects_6 = ({ let q: &dyn Query = g; ((moved_5.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_6 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_6 { let controller = g.state().obj(o).owner; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Clive, Ifrit's Dominant", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Clive enters, you may discard your hand, then draw cards equal to your devotion to red.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{4}{R}{R}, {T}: Exile Clive, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0).between(0, 1)];
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let first_0 = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied(); let second_0 = ({ let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied();
if let (Some(first), Some(second)) = (first_0, second_0) { g.fight(first, second); }
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::R, mana_2_0);}
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Lore))) >= (3i64)) } {
let objects_3 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let player_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_4 = ({ let q: &dyn Query = g; ((moved_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let rows_4: Vec<(ObjectId, PlayerId, bool)> = objects_4.iter().map(|&o| (o, player_4, false)).collect(); let came_4 = g.put_all_onto_battlefield_as(&rows_4)?; came_4.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_5.extend(moved_3.clone());
hoisted_6.extend(moved_4.clone());
hoisted_7.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_8.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_16: Vec<ObjectId> = Vec::new();
let mut hoisted_15: Vec<ObjectId> = Vec::new();
let mut hoisted_14: Vec<ObjectId> = Vec::new();
let mut hoisted_13: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mana_players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_10_0 = { let q: &dyn Query = g; (4i64).max(0) as u32 };
for player in mana_players_10 {
g.add_mana(player, Mana::R, mana_10_0);}
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| (i64::from(q.state().obj(candidate).counters(CounterKind::Lore))) >= (3i64)) } {
let objects_11 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_11 = g.exile_all(&objects_11);
g.end_sentence();
let player_12 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_12 = ({ let q: &dyn Query = g; ((moved_11.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_12 = { let rows_12: Vec<(ObjectId, PlayerId, bool)> = objects_12.iter().map(|&o| (o, player_12, false)).collect(); let came_12 = g.put_all_onto_battlefield_as(&rows_12)?; came_12.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_13.extend(moved_11.clone());
hoisted_14.extend(moved_12.clone());
hoisted_15.extend({ let q: &dyn Query = g; objects_11.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
hoisted_16.extend({ let q: &dyn Query = g; objects_12.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Ifrit, Warden of Inferno", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Saga,Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(8)), power: Some(9), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Lunge — Ifrit fights up to one other target creature.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "II, III — Brimstone — Add {R}{R}{R}{R}. If Ifrit has three or more lore counters on it, exile it, then return it to the battlefield .").targets(&[]),chapter(3, ability_0_2, "II, III — Brimstone — Add {R}{R}{R}{R}. If Ifrit has three or more lore counters on it, exile it, then return it to the battlefield .").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
