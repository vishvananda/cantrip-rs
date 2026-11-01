// Esper Terra // Terra, Magical Adept
// status: planned
// requires: operation PutCountersEach
// (As this Saga enters and after your draw step, add a lore counter.)
// I, II, III — Create a token that's a copy of target nonlegendary enchantment you control. It gains haste. If it's a Saga, put up to three lore counters on it. Sacrifice it at the beginning of your next end step.
// IV — Add {W}{W}, {U}{U}, {B}{B}, {R}{R}, and {G}{G}. Exile Esper Terra, then return it to the battlefield (front face up).
// Flying
// When Terra enters, mill five cards. Put up to one enchantment card milled this way into your hand.
// Trance — {4}{R}{G}, {T}: Exile Terra, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (5i64).max(0) as u32 }; let mills_0: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_0)).collect(); let milled_0 = g.mill_together(&mills_0);
g.end_sentence();
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; (milled_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Enchantment)).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::PutIntoHand, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
let objects_2 = { let q: &dyn Query = g; selected_1.clone() }; let moved_2 = g.return_to_hand(&objects_2);
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_3 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_3 = g.exile_all(&objects_3);
g.end_sentence();
let objects_4 = ({ let q: &dyn Query = g; ((moved_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_4 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_4 { let controller = g.state().obj(o).owner; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Terra, Magical Adept", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Wizard,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Terra enters, mill five cards. Put up to one enchantment card milled this way into your hand.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "Trance — {4}{R}{G}, {T}: Exile Terra, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Enchantment) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_0_target_0)];
fn delayed_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; about(q, item, 0) }; let players_5 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_5: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_5 { let controller = g.controller(object); if players_5.contains(&controller) && g.can_sacrifice(controller, object) { picks_5.push((controller, object)); } }
let sacrificed_5 = g.sacrifice_each(&picks_5);
g.end_sentence();
 Ok(()) }
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_0 = match original_0 { Some(original) => g.create_token_copies(original, player_0, count_0)?, None => Vec::new() };
g.end_sentence();
let objects_1 = { let q: &dyn Query = g; (created_0.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
planned!("operation-put-counters-each" => cantrip_mechanics::counters::put_on_each(g, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((created_0.clone()).contains(&id) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }, CounterKind::Lore, { let q: &dyn Query = g; (3i64).max(0) as i16 }, true)?);
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_0.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: Some({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }) }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), &about); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Enchantment) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_1_target_0)];
fn delayed_10(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_11 = { let q: &dyn Query = g; about(q, item, 0) }; let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_11: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_11 { let controller = g.controller(object); if players_11.contains(&controller) && g.can_sacrifice(controller, object) { picks_11.push((controller, object)); } }
let sacrificed_11 = g.sacrifice_each(&picks_11);
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_6 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_6 = match original_6 { Some(original) => g.create_token_copies(original, player_6, count_6)?, None => Vec::new() };
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; (created_6.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_7 = { let q: &dyn Query = g; Some(asker.source) }; let duration_7 = Duration::Indefinite;
let kind_7_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_7 { cantrip_api::instruction(g, |g| { for &object in &objects_7 { g.add_effect(source, object, kind_7_0_0, duration_7);  } }); }
g.end_sentence();
planned!("operation-put-counters-each" => cantrip_mechanics::counters::put_on_each(g, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((created_6.clone()).contains(&id) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }, CounterKind::Lore, { let q: &dyn Query = g; (3i64).max(0) as i16 }, true)?);
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_6.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: Some({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }) }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 1), &about); } }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((!(q.chars(id).supertypes.legendary) && q.chars(id).is(CardType::Enchantment) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_0_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_2_target_0)];
fn delayed_16(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_17 = { let q: &dyn Query = g; about(q, item, 0) }; let players_17 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_17: Vec<(PlayerId, ObjectId)> = Vec::new();
for &object in &objects_17 { let controller = g.controller(object); if players_17.contains(&controller) && g.can_sacrifice(controller, object) { picks_17.push((controller, object)); } }
let sacrificed_17 = g.sacrifice_each(&picks_17);
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let original_12 = { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()) }; let count_12 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let player_12 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let created_12 = match original_12 { Some(original) => g.create_token_copies(original, player_12, count_12)?, None => Vec::new() };
g.end_sentence();
let objects_13 = { let q: &dyn Query = g; (created_12.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_13 = { let q: &dyn Query = g; Some(asker.source) }; let duration_13 = Duration::Indefinite;
let kind_13_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_13 { cantrip_api::instruction(g, |g| { for &object in &objects_13 { g.add_effect(source, object, kind_13_0_0, duration_13);  } }); }
g.end_sentence();
planned!("operation-put-counters-each" => cantrip_mechanics::counters::put_on_each(g, asker.source, { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((created_12.clone()).contains(&id) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Saga)))).collect::<Vec<ObjectId>>() }, CounterKind::Lore, { let q: &dyn Query = g; (3i64).max(0) as i16 }, true)?);
g.end_sentence();
{ let mut about: Vec<ObjectId> = Vec::new(); about.extend({ let q: &dyn Query = g; created_12.clone() }); about.push(ObjectId::NONE);  { g.delayed_trigger_about(DelayedWhen::Step { step: Step::End, on_turn_of: Some({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }) }, item.object, item.controller, AbilityRef::delayed(item.ability.card, 2), &about); } }
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_19 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_19_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let mana_19_1 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let mana_19_2 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let mana_19_3 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
let mana_19_4 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_19 {
g.add_mana(player, Mana::W, mana_19_0);g.add_mana(player, Mana::U, mana_19_1);g.add_mana(player, Mana::B, mana_19_2);g.add_mana(player, Mana::R, mana_19_3);g.add_mana(player, Mana::G, mana_19_4);}
g.end_sentence();
let objects_20 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_20 = g.exile_all(&objects_20);
g.end_sentence();
let player_21 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_21 = ({ let q: &dyn Query = g; ((moved_20.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_21 = { let rows_21: Vec<(ObjectId, PlayerId, bool)> = objects_21.iter().map(|&o| (o, player_21, false)).collect(); let came_21 = g.put_all_onto_battlefield_as(&rows_21)?; came_21.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
fn about(q: &dyn Query, item: &StackItem, part: usize) -> Vec<ObjectId> { q.event_objects(item).split(|&id| id == ObjectId::NONE).nth(part).map_or_else(Vec::new, <[ObjectId]>::to_vec) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Esper Terra", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Saga,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(24)), power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II, III — Create a token that's a copy of target nonlegendary enchantment you control. It gains haste. If it's a Saga, put up to three lore counters on it. Sacrifice it at the beginning of your next end step.").targets(ABILITY_0_0_TARGETS),chapter(2, ability_0_1, "I, II, III — Create a token that's a copy of target nonlegendary enchantment you control. It gains haste. If it's a Saga, put up to three lore counters on it. Sacrifice it at the beginning of your next end step.").targets(ABILITY_0_1_TARGETS),chapter(3, ability_0_2, "I, II, III — Create a token that's a copy of target nonlegendary enchantment you control. It gains haste. If it's a Saga, put up to three lore counters on it. Sacrifice it at the beginning of your next end step.").targets(ABILITY_0_2_TARGETS),chapter(4, ability_0_3, "IV — Add {W}{W}, {U}{U}, {B}{B}, {R}{R}, and {G}{G}. Exile Esper Terra, then return it to the battlefield .").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(delayed_4, ""),delayed_trigger(delayed_10, ""),delayed_trigger(delayed_16, "")], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
