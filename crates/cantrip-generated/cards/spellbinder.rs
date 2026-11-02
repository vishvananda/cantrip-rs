// Spellbinder
// status: ready
// Imprint — When this Equipment enters, you may exile an instant card from your hand.
// Whenever equipped creature deals combat damage to a player, you may copy the exiled card. If you do, you may cast the copy without paying its mana cost.
// Equip {4}
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
{
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).is(CardType::Instant)).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::Exile, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
if !selected_0.is_empty() { 
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let moved_1 = { let by = g.linked(item); g.exile_with(&objects_1, by, false) };
g.end_sentence();
hoisted_2.extend(moved_1.clone());
hoisted_3.extend(selected_0.clone());
hoisted_4.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_5 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_5 = cantrip_mechanics::effects::ask_may(g, payer_5, asker.source, Purpose::Optional)?; if paid_5 {
let player_6 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_6 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let times_6 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_6: Vec<ObjectId> = Vec::new(); for _ in 0..times_6 { for &card in &originals_6 { copies_6.push(g.create_copy_in_exile(card, player_6)); } }
g.end_sentence();
hoisted_7.extend(copies_6.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_5 as i64) != 0) } {
let caster_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_8: Vec<ObjectId> = { let q: &dyn Query = g; (hoisted_7.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_8: u32 = 1; let mut played_8: Vec<ObjectId> = Vec::new();
while left_8 > 0 {
let mut open_8: Vec<ObjectId> = Vec::new(); for &c in &pool_8 { let ways = g.cast_ways_during_resolution_by(c, caster_8); if ways.iter().any(|w| true) { open_8.push(c); } }
let picked_8 = if open_8.is_empty() { None } else if open_8.len() > 1 || (true && (!false || pool_8.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_8, asker.source, Purpose::CastFree, pool_8.clone(), open_8.clone(), 1, true)?.first().copied() } else if cantrip_mechanics::effects::ask_may(g, caster_8, open_8[0], Purpose::CastFree)? { Some(open_8[0]) } else { None };
let Some(card_8) = picked_8 else { break; };
let ways_8: Vec<CastWay> = g.cast_ways_during_resolution_by(card_8, caster_8).into_iter().filter(|w| true).collect();
let done_8 = { let way = cantrip_mechanics::choose::cast_way(g, caster_8, card_8, ways_8)?.map(|w| w.way); g.cast_during_resolution_as(card_8, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_8), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_8 { left_8 -= 1; played_8.push(g.state().obj(card_8).became.unwrap_or(card_8)); } else { break; } if done_8 || !true { pool_8.retain(|&c| c != card_8); }
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))) }
static ABILITY_2_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_2_target_0)];
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let attachments_9 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_9 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()) }; if let Some(host) = host_9 { cantrip_api::instruction(g, |g| { for attachment in attachments_9 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Spellbinder", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Equipment]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Imprint — When this Equipment enters, you may exile an instant card from your hand.").targets(&[]),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever equipped creature deals combat damage to a player, you may copy the exiled card. If you do, you may cast the copy without paying its mana cost.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "Equip {4}").targets(ABILITY_2_TARGETS).equip().sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
