// Bladecoil Serpent
// status: ready
// When this creature enters, for each {U}{U} spent to cast it, draw a card.
// When this creature enters, for each {B}{B} spent to cast it, each opponent discards a card.
// When this creature enters, for each {R}{R} spent to cast it, it gets +1/+0 and gains trample and haste until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((item.subject.object().map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::U].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })).div_euclid(2)).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; ((item.subject.object().map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::B].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })).div_euclid(2)).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in g.state().apnap().filter(|p| players_1.contains(p)).collect::<Vec<PlayerId>>() { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((item.subject.object().map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::R].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })).div_euclid(2)).clamp(-32768, 32767) as i16, ((0i64 * (item.subject.object().map_or(0, |o: ObjectId| { let spent = q.obj(o).spent; [Mana::R].into_iter().map(|mana| i64::from(spent.get(mana))).sum::<i64>() })).div_euclid(2))) as i16) };
let kind_2_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Trample])) };
let kind_2_2_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2); g.add_effect(source, object, kind_2_1_0, duration_2); g.add_effect(source, object, kind_2_2_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Bladecoil Serpent", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Serpent]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, for each {U}{U} spent to cast it, draw a card.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, for each {B}{B} spent to cast it, each opponent discards a card.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, for each {R}{R} spent to cast it, it gets +1/+0 and gains trample and haste until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
