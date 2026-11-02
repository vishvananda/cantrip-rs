// Bloodthirsty Adversary
// status: ready
// Haste
// When this creature enters, you may pay {2}{R} any number of times. When you pay this cost one or more times, put that many +1/+1 counters on this creature, then exile up to that many target instant and/or sorcery cards with mana value 3 or less from your graveyard and copy them. You may cast any number of the copies without paying their mana costs.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn reflexive_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Graveyard) && !q.state().obj(id).token && (((q.chars(id).is(CardType::Instant) || q.chars(id).is(CardType::Sorcery)) && (i64::from(q.mana_value(id))) <= (3i64) && (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))))) }
static REFLEXIVE_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(reflexive_4_target_0).between(0, TargetSpec::COUNTED)];
fn reflexive_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut counters_5: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (i64::from(item.x)).min(i16::MAX.into()) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_5.push((object, CounterKind::PlusOne, amount)); } }
let had_5: Vec<_> = counters_5.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_5); let applied_5: Vec<ObjectId> = counters_5.iter().zip(had_5).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; t_0.iter().filter_map(|t| t.object()).collect::<Vec<ObjectId>>() }; let moved_6 = g.exile_all(&objects_6);
g.end_sentence();
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let originals_7 = { let q: &dyn Query = g; moved_6.clone() }; let times_7 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut copies_7: Vec<ObjectId> = Vec::new(); for _ in 0..times_7 { for &card in &originals_7 { copies_7.push(g.create_copy_in_exile(card, player_7)); } }
g.end_sentence();
let caster_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut pool_8: Vec<ObjectId> = { let q: &dyn Query = g; (copies_7.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let mut left_8: u32 = 4294967295; let mut played_8: Vec<ObjectId> = Vec::new();
while left_8 > 0 {
let mut open_8: Vec<ObjectId> = Vec::new(); for &c in &pool_8 { let ways = g.cast_ways_during_resolution_by(c, caster_8); if ways.iter().any(|w| true) { open_8.push(c); } }
let picked_8 = if open_8.is_empty() { None } else if open_8.len() > 1 || (true && (!true || pool_8.len() > 1)) { cantrip_mechanics::choose::cards_from(g, caster_8, asker.source, Purpose::CastFree, pool_8.clone(), open_8.clone(), 1, true)?.first().copied() } else if !true || cantrip_mechanics::effects::ask_may(g, caster_8, open_8[0], Purpose::CastFree)? { Some(open_8[0]) } else { None };
let Some(card_8) = picked_8 else { break; };
let ways_8: Vec<CastWay> = g.cast_ways_during_resolution_by(card_8, caster_8).into_iter().filter(|w| true).collect();
let done_8 = if ways_8.is_empty() { g.play_land_during_resolution(card_8)?.is_some() } else { let way = cantrip_mechanics::choose::cast_way(g, caster_8, card_8, ways_8)?.map(|w| w.way); g.cast_during_resolution_as(card_8, CastTerms { instead: Some(ManaCost::ZERO), way, caster: Some(caster_8), exile_instead: ExileInstead::Never, ..CastTerms::default() })? };
if done_8 { left_8 -= 1; played_8.push(g.state().obj(card_8).became.unwrap_or(card_8)); } else if false { break; } if done_8 || !(true && true) { pool_8.retain(|&c| c != card_8); }
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
'optional_0: {
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times_2 = g.may_pay_times(payer_2, asker.source, asker.source, ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, cantrip_api::ANY_NUMBER_OF_TIMES, &PAY_QUESTION)?; let paid_2 = times_2 > 0; let bound_2: i64 = i64::from(times_2);
if !paid_2 { break 'optional_0; }
if paid_2 && true { g.reflexive_trigger_counted(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), Target::None, (bound_2).clamp(0, i64::from(u16::MAX)) as u16); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Bloodthirsty Adversary", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, you may pay {2}{R} any number of times. When you pay this cost one or more times, put that many +1/+1 counters on this creature, then exile up to that many target instant and/or sorcery cards with mana value 3 or less from your graveyard and copy them. You may cast any number of the copies without paying their mana costs.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_4, "When you pay this cost one or more times, put that many +1/+1 counters on this creature, then exile up to that many target instant and/or sorcery cards with mana value 3 or less from your graveyard and copy them. You may cast any number of the copies without paying their mana costs.").targets(REFLEXIVE_4_TARGETS)], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
