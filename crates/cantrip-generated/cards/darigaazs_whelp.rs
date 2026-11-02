// Darigaaz's Whelp
// status: planned
// requires: engine: no perpetual effects
// Kicker {1}{R}
// Flying
// Whenever you draw a Dragon card, it perpetually gets +1/+1.
// When this creature enters, if it was kicked, seek a Dragon card. This creature and that Dragon card each perpetually get +1/+1.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Drew { player, card } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_1 { let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
 Ok(()) }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| q.state().obj(candidate).kicked > 0) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut pool_2 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon))).collect::<Vec<ObjectId>>() }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as usize }; let mut picked_2: Vec<ObjectId> = Vec::new(); while picked_2.len() < count_2 && !pool_2.is_empty() { let at = g.choose_at_random(pool_2.len()); picked_2.push(pool_2.remove(at)); }
let objects_3 = { let q: &dyn Query = g; picked_2.clone() }; let moved_3 = g.return_to_hand(&objects_3);
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_5: Vec<ObjectId> = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_5 { let kind_5_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_5_0); });
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_7: Vec<ObjectId> = { let q: &dyn Query = g; (moved_3.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_7 { let kind_7_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_7_0); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Darigaaz's Whelp", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DREW.0), ability_2_event), ability_2, "Whenever you draw a Dragon card, it perpetually gets +1/+1.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When this creature enters, if it was kicked, seek a Dragon card. This creature and that Dragon card each perpetually get +1/+1.").targets(&[]).condition(ability_3_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, kicker: kicker(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
