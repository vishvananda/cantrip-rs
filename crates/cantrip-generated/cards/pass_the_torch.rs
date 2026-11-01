// Pass the Torch
// status: planned
// requires: engine: no perpetual effects
// requires: losing ability ThisAbility
// Pass the Torch deals 2 damage to any target. Choose a creature card in your hand. It perpetually gains "Whenever this creature deals combat damage to a player, you may cast target card named Pass the Torch from your graveyard without paying its mana cost. If you do, this creature perpetually loses this ability".
#[allow(unused_imports)] use cantrip_api::*;
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[cantrip_mechanics::targets::ANY];
#[allow(unused_variables)] fn perpetual_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && ((q.chars(id).name == "Pass the Torch" && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static PERPETUAL_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(perpetual_4_target_0)];
#[allow(unused_variables)] fn perpetual_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let payer_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_5 = cantrip_mechanics::effects::ask_may(g, payer_5, asker.source, Purpose::Optional)?; if paid_5 {
let caster_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_6: Vec<ObjectId> = { let q: &dyn Query = g; (t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_6: u32 = 1; let mut played_6: Vec<ObjectId> = Vec::new();
while left_6 > 0 {
let mut open_6: Vec<ObjectId> = Vec::new(); for &c in &pool_6 { let ways = g.cast_ways_during_resolution_by(c, caster_6); if ways.iter().any(|w| true) { open_6.push(c); } }
let picked_6 = if open_6.is_empty() { None } else if open_6.len() > 1 || (false && (!false || pool_6.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_6, asker.source, Purpose::CastFree, pool_6.clone(), open_6.clone(), 1, false)?.first().copied() } else if !false || cantrip_mechanics::effects::ask_may(g, caster_6, open_6[0], Purpose::CastFree)? { Some(open_6[0]) } else { None };
let Some(card_6) = picked_6 else { break; };
let ways_6: Vec<CastWay> = g.cast_ways_during_resolution_by(card_6, caster_6).into_iter().filter(|w| true).collect();
let done_6 = if ways_6.is_empty() { g.play_land_during_resolution(card_6)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_6, card_6, ways_6)?.map(|w| w.way); g.cast_during_resolution_as(card_6, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_6), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_6 { left_6 -= 1; played_6.push(g.state().obj(card_6).became.unwrap_or(card_6)); } else if 1i64 == 1 { break; } if done_6 || !(false && false) { pool_6.retain(|&c| c != card_6); }
}
g.end_sentence();
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
planned!("engine-no-perpetual-effects" => let objects_8: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let kind_8_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(planned_value!("losing ability ThisAbility", EffectKind::Pump(0, 0) => EffectKind::LosesAbility(Lost::ThisAbility))) };
for &object in &objects_8 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_8_0); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut done_0: i64 = 0; let source_0 = { let q: &dyn Query = g; Some(asker.source) }; let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u16 }; let recipients_0 = { let q: &dyn Query = g; t_0.clone() }; if let Some(source) = source_0 { let batch: Vec<(ObjectId, Target, u16)> = recipients_0.iter().map(|t| (source, *t, amount_0)).collect(); done_0 += g.damage_together(&batch).iter().map(|&n| i64::from(n)).sum::<i64>(); }
g.end_sentence();
let selected_1_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; (selected_1.clone()).first().copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) };
for &object in &objects_3 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Pass the Torch", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), perpetual_4_event), perpetual_4, "Whenever this creature deals combat damage to a player, you may cast target card named Pass the Torch from your graveyard without paying its mana cost. If you do, this creature perpetually loses this ability").targets(PERPETUAL_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
