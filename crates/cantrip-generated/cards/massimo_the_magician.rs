// Massimo, the Magician
// status: ready
// Whenever Massimo or another creature you control enters, exile up to one target instant or sorcery card with mana value 1 from your graveyard. If a card is exiled this way, that creature gains "Whenever this creature deals combat damage to a player, copy the exiled card. You may cast the copy without paying its mana cost."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (i64::from(q.mana_value(id))) == (1i64) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 1)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn granted_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn granted_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_3 = { let q: &dyn Query = g; (q.exiled_with(q.linked(item))).first().copied().into_iter().collect::<Vec<ObjectId>>() }; let times_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_3: Vec<ObjectId> = Vec::new(); for _ in 0..times_3 { for &card in &originals_3 { copies_3.push(g.create_copy_in_exile(card, player_3)); } }
g.end_sentence();
let caster_4 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_4: Vec<ObjectId> = { let q: &dyn Query = g; (copies_3.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_4: u32 = 1; let mut played_4: Vec<ObjectId> = Vec::new();
while left_4 > 0 {
let mut open_4: Vec<ObjectId> = Vec::new(); for &c in &pool_4 { let ways = g.cast_ways_during_resolution_by(c, caster_4); if ways.iter().any(|w| true) { open_4.push(c); } }
let picked_4 = if open_4.is_empty() { None } else if open_4.len() > 1 || (true && (!true || pool_4.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_4, asker.source, Purpose::CastFree, pool_4.clone(), open_4.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_4, open_4[0], Purpose::CastFree)? { Some(open_4[0]) } else { None };
let Some(card_4) = picked_4 else { break; };
let ways_4: Vec<CastWay> = g.cast_ways_during_resolution_by(card_4, caster_4).into_iter().filter(|w| true).collect();
let done_4 = if ways_4.is_empty() { g.play_land_during_resolution(card_4)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_4, card_4, ways_4)?.map(|w| w.way); g.cast_during_resolution_as(card_4, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_4), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_4 { left_4 -= 1; played_4.push(g.state().obj(card_4).became.unwrap_or(card_4)); } else if 1i64 == 1 { break; } if done_4 || !(true && true) { pool_4.retain(|&c| c != card_4); }
}
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; let moved_0 = g.exile_all(&objects_0);
let paid_0 = moved_0.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
g.end_sentence();
if { let q: &dyn Query = g; (((moved_0.clone()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
let objects_1 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::Indefinite;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
hoisted_5.extend(objects_1.clone());
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Massimo, the Magician", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever Massimo or another creature you control enters, exile up to one target instant or sorcery card with mana value 1 from your graveyard. If a card is exiled this way, that creature gains \"Whenever this creature deals combat damage to a player, copy the exiled card. You may cast the copy without paying its mana cost.\"").targets(ABILITY_0_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), granted_2_event), granted_2, "Whenever this creature deals combat damage to a player, copy the exiled card. You may cast the copy without paying its mana cost.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
