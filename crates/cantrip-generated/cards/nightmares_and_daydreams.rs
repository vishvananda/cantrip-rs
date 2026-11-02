// Nightmares and Daydreams
// status: ready
// (As this Saga enters and after your draw step, add a lore counter. Sacrifice after IV.)
// I, II, III — Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.
// IV — Draw a card. If a graveyard has twenty or more cards in it, draw three cards instead.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_1_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static WATCH_1_TARGETS: &[cantrip_api::TargetSpec] = &[target(watch_1_target_0)];
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_2 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as u32 }; let mills_2: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_2)).collect(); let milled_2 = g.mill_together(&mills_2);
g.end_sentence();
 Ok(()) }
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }), source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static WATCH_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(watch_4_target_0)];
#[allow(unused_variables)] fn watch_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn watch_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_5 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as u32 }; let mills_5: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_5)).collect(); let milled_5 = g.mill_together(&mills_5);
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }), source, item.controller, AbilityRef::delayed(item.ability.card, 1), Target::None); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn watch_7_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static WATCH_7_TARGETS: &[cantrip_api::TargetSpec] = &[target(watch_7_target_0)];
#[allow(unused_variables)] fn watch_7_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) { return Some(Fired { subject, data }); } }
 None }
fn watch_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let amount_8 = { let q: &dyn Query = g; (item.subject.object().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as u32 }; let mills_8: Vec<(PlayerId, u32)> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() }.into_iter().map(|p| (p, amount_8)).collect(); let milled_8 = g.mill_together(&mills_8);
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }), source, item.controller, AbilityRef::delayed(item.ability.card, 2), Target::None); }
g.end_sentence();
 Ok(()) }
fn ability_0_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) >= (20i64)) } {
let amount_9 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_9 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_9 { g.draw(player, amount_9); }
g.end_sentence();
} else {
let amount_10 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_10 { g.draw(player, amount_10); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Nightmares and Daydreams", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I, II, III — Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.").targets(&[]),chapter(2, ability_0_1, "I, II, III — Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.").targets(&[]),chapter(3, ability_0_2, "I, II, III — Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.").targets(&[]),chapter(4, ability_0_3, "IV — Draw a card. If a graveyard has twenty or more cards in it, draw three cards instead.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_1_event), watch_1, "Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.").targets(WATCH_1_TARGETS),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_4_event), watch_4, "Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.").targets(WATCH_4_TARGETS),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), watch_7_event), watch_7, "Until your next turn, whenever you cast an instant or sorcery spell, target player mills cards equal to that spell's mana value.").targets(WATCH_7_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
