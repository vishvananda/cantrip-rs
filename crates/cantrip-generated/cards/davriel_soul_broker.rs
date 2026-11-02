// Davriel, Soul Broker
// status: planned
// requires: a choice among options offered at random
// requires: a spell costing colored mana more
// requires: engine: no perpetual effects
// requires: operation Conjure
// [+1]: Until your next turn, whenever an opponent attacks you and/or planeswalkers you control, they discard a card. If they can't, they sacrifice an attacking creature.
// [−2]: Accept one of Davriel's offers, then accept one of Davriel's conditions.
// [−3]: Target creature an opponent controls perpetually gets -3/-3.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn watch_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if true && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id)) && match *defender { Target::Player(p) => Some(asker.controller) == Some(p), Target::Object(o) => q.chars(o).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(o)), _ => false } { return Some(Fired { subject, data }); } }
 None }
fn watch_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_3)?); } let discarded_3 = g.discard_together(&cards_3);
let paid_2 = !discarded_3.clone().is_empty();
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_2 as i64) != 0)) } {
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.state().attackers.iter().any(|a| a.id == id) || q.was_attacking(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let players_4 = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_4: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_4 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_4 { let eligible: Vec<ObjectId> = objects_4.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_4.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_4.clone(), &[]))? }; for object in taken { picks_4.push((player, object)); } }
let sacrificed_4 = g.sacrifice_each(&picks_4);
g.end_sentence();
hoisted_5.extend(sacrificed_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(source) = Some(item.object) { g.delayed_trigger(DelayedWhen::UntilNextTurnOf({ let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }), source, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None); }
g.end_sentence();
 Ok(()) }
fn offer_7(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_8 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let players_8 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.draw(player, amount_8); }
g.end_sentence();
 Ok(()) }
fn offer_9(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_10: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_10: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_10.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Manor Guardian"), count_10, ConjureTo::Hand)?)); }
g.end_sentence();
 Ok(()) }
fn offer_11(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_12 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let count_12 = { let q: &dyn Query = g; (2i64).max(0) as usize }; let mut picked_12: Vec<ObjectId> = Vec::new(); while picked_12.len() < count_12 && !pool_12.is_empty() { let at = g.choose_at_random(pool_12.len()); picked_12.push(pool_12.remove(at)); }
let objects_13 = { let q: &dyn Query = g; picked_12.clone() }; let moved_13 = g.return_to_hand(&objects_13);
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_15: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (moved_13.clone()).contains(&id)).collect::<Vec<ObjectId>>() };
for &object in &objects_15 { let kind_15_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_15_0); });
g.end_sentence();
 Ok(()) }
fn offer_16(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_17 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).is(CardType::Creature) && (i64::from(q.mana_value(id))) == ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.mana_value(o))).max().unwrap_or(0)))).collect::<Vec<ObjectId>>() }; let count_17 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_17: Vec<ObjectId> = Vec::new(); while picked_17.len() < count_17 && !pool_17.is_empty() { let at = g.choose_at_random(pool_17.len()); picked_17.push(pool_17.remove(at)); }
let player_18 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_18 = ({ let q: &dyn Query = g; picked_17.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_18 = { let rows_18: Vec<(ObjectId, PlayerId, bool)> = objects_18.iter().map(|&o| (o, player_18, false)).collect(); let came_18 = g.put_all_onto_battlefield_as(&rows_18)?; came_18.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
pub mod emblem_20 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((2i64) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, ability_0_affects, ability_0_apply_0, "Static: Pump.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_19(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_20::CARD); }
g.end_sentence();
 Ok(()) }
pub mod emblem_22 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && true { { let mut less = ManaCost::ZERO; less.symbols[2] += (1i64) as u8; CostDelta::less_mana(less) } } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_21(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_22::CARD); }
g.end_sentence();
 Ok(()) }
pub mod emblem_24 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Davriel)) && (*so_far).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(affected))) }
fn granted_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Static: GrantAbility.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(2)), granted_0, "Draw 1 cards.").targets(&[]).sorcery_speed()], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_23(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_24::CARD); }
g.end_sentence();
 Ok(()) }
pub mod emblem_26 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Drew { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.gain_life(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DREW.0), ability_0_event), ability_0, "Whenever a player draws a card, Gain 2 life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_25(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_26::CARD); }
g.end_sentence();
 Ok(()) }
fn offer_27(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_28 = { let q: &dyn Query = g; (6i64).max(0) as i32 }; let players_28 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_28 { g.lose_life(player, amount_28); }
g.end_sentence();
 Ok(()) }
fn offer_29(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_30_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_30_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_30_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_30 = { cantrip_mechanics::choose::select(g, selected_30_player, asker.source, Purpose::Exile, selected_30_candidates, cantrip_mechanics::select::Selection::of(selected_30_count, &[]))? };
let objects_31 = { let q: &dyn Query = g; selected_30.clone() }; let moved_31 = g.exile_all(&objects_31);
g.end_sentence();
if { let q: &dyn Query = g; ((objects_31.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).len() as i64) < (2i64) } {
let amount_32 = { let q: &dyn Query = g; ((2i64 - (objects_31.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).len() as i64)).max(0) as u32 }; let players_32 = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let drawing_32: Vec<PlayerId> = players_32.into_iter().collect(); let players_32: Vec<PlayerId> = g.state().apnap().filter(|p| drawing_32.contains(p)).collect();
for player in players_32 { g.draw(player, amount_32); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn offer_33(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_34 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| true).collect::<Vec<ObjectId>>() }; let players_34 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_34: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_34 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; for player in players_34 { let eligible: Vec<ObjectId> = objects_34.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_34.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_34.clone(), &[]))? }; for object in taken { picks_34.push((player, object)); } }
let sacrificed_34 = g.sacrifice_each(&picks_34);
g.end_sentence();
 Ok(()) }
fn offer_35(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_37: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && ({ let other = Some(asker.controller); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }).contains(&q.controller(id)))).collect::<Vec<ObjectId>>() };
for &object in &objects_37 { let kind_37_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_37_0); });
g.end_sentence();
 Ok(()) }
pub mod emblem_39 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((-1i64) as i16, (0i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::PtModify, ability_0_affects, ability_0_apply_0, "Static: Pump.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_38(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_39::CARD); }
g.end_sentence();
 Ok(()) }
pub mod emblem_41 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0_rule(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { let asker = &Asker::new(source, q.controller(source)); let Costed::Spell(spell) = costed else { return CostDelta::NONE; }; if player == asker.controller && true { planned_value!("a spell costing colored mana more", CostDelta::NONE => CostDelta::more_mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })) } else { CostDelta::NONE } }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, cost_changes: &[ability_0_0_rule], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_40(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_41::CARD); }
g.end_sentence();
 Ok(()) }
pub mod emblem_43 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Drew { player, .. } = event { let subject = Target::Player(*player); let data = [0u32; 2]; if Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((2i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DREW.0), ability_0_event), ability_0, "Whenever a player draws a card, Return the chosen permanents to their owners' hands.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_42(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_43::CARD); }
g.end_sentence();
 Ok(()) }
pub mod emblem_45 {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::Upkeep) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 2147483647) as i32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker emblem", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_0_event), ability_0, "At the beginning of this step, Lose life.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn offer_44(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::tokens::create_emblem(g, player, &emblem_45::CARD); }
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("a-choice-among-options-offered-at-random" => cantrip_mechanics::choose::among_random(g, item, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, { let q: &dyn Query = g; 3i64 }, &[offer_7, offer_9, offer_11, offer_16, offer_19, offer_21, offer_23, offer_25])?);
g.end_sentence();
planned!("a-choice-among-options-offered-at-random" => cantrip_mechanics::choose::among_random(g, item, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, { let q: &dyn Query = g; 3i64 }, &[offer_27, offer_29, offer_33, offer_35, offer_38, offer_40, offer_42, offer_44])?);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("engine-no-perpetual-effects" => let objects_47: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() };
for &object in &objects_47 { let kind_47_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((-3i64) as i16, (-3i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_47_0); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Davriel, Soul Broker", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Planeswalker]), subtypes: SubtypeSet::of(&[Subtype::Davriel]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: None, toughness: None, loyalty: Some(4), defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(1)), ability_0, "+1: Until your next turn, whenever an opponent attacks you and/or planeswalkers you control, they discard a card. If they can't, they sacrifice an attacking creature.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-2)), ability_1, "−2: Accept one of Davriel's offers, then accept one of Davriel's conditions.").targets(&[]).sorcery_speed(),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(loyalty(-3)), ability_2, "−3: Target creature an opponent controls perpetually gets -3/-3.").targets(ABILITY_2_TARGETS).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), watch_1_event), watch_1, "Until your next turn, whenever an opponent attacks you and/or planeswalkers you control, they discard a card.").targets(&[]).once_per_batch().batch_by_data()], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&emblem_20::CARD,&emblem_22::CARD,&emblem_24::CARD,&emblem_26::CARD,&emblem_39::CARD,&emblem_41::CARD,&emblem_43::CARD,&emblem_45::CARD];
