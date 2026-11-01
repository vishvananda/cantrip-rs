// _____ _____ Rocketship
// status: planned
// requires: operation ChooseValueWith
// requires: operation PutStickers
// requires: stickers
// Flying
// When this Vehicle enters, you may put up to two name stickers on it.
// Whenever this Vehicle attacks, choose a letter. This Vehicle gets +1/+1 until end of turn for each name sticker on it that begins with the chosen letter.
// Crew 2
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let decider_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_0, asker.source, Purpose::Optional)? {
planned!("operation-put-stickers" => cantrip_mechanics::stickers::stickers(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, "Some(Name)", &{ let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }, Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), false, None)?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-value-with" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Letter, ChoiceTerms { options: &[], except: &[], count: None, secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
let objects_3 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let duration_3 = Duration::EndOfTurn;
let kind_3_0_0 = { let q: &dyn Query = g; EffectKind::Pump((planned_value!("stickers", 0i64 => cantrip_mechanics::stickers::measure(q, &Some(asker.source).into_iter().collect::<Vec<ObjectId>>(), StickerMeasure::NamesBeginningWith))).clamp(-32768, 32767) as i16, (planned_value!("stickers", 0i64 => cantrip_mechanics::stickers::measure(q, &Some(asker.source).into_iter().collect::<Vec<ObjectId>>(), StickerMeasure::NamesBeginningWith))).clamp(-32768, 32767) as i16) };
if let Some(source) = source_3 { cantrip_api::instruction(g, |g| { for &object in &objects_3 { g.add_effect(source, object, kind_3_0_0, duration_3);  } }); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "_____ _____ Rocketship", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this Vehicle enters, you may put up to two name stickers on it.").targets(&[]),triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever this Vehicle attacks, choose a letter. This Vehicle gets +1/+1 until end of turn for each name sticker on it that begins with the chosen letter.").targets(&[]),cantrip_mechanics::crew::crew(2, "Crew 2")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
