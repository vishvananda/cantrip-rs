// Ambassador Blorpityblorpboop
// status: planned
// requires: engine: no tickets
// requires: operation PutSticker
// requires: stickers
// When Ambassador Blorpityblorpboop enters, you get {TK}{TK}{TK}, then you may put a sticker on a nonland permanent you own.
// At the beginning of each combat, you may have Ambassador Blorpityblorpboop's base power become equal to the total power of all stickers on permanents you control and its base toughness become equal to those stickers' total toughness.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-tickets" => { let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = { let q: &dyn Query = g; (3i64).max(0) as i32 }; g.change_tickets(player, amount); });
g.end_sentence();
let decider_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_1, asker.source, Purpose::Optional)? {
let player_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && Some(asker.controller) == Some(q.state().obj(id).owner))).collect::<Vec<ObjectId>>() };
planned!("operation-put-sticker" => cantrip_mechanics::stickers::sticker(g, player_2, asker.source, &objects_2)?);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::BeginCombat) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let decider_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; if cantrip_mechanics::effects::ask_may(g, decider_3, asker.source, Purpose::Optional)? {
let objects_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::Indefinite;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((planned_value!("stickers", 0i64 => cantrip_mechanics::stickers::measure(q, &q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>(), StickerMeasure::TotalPower))).clamp(-32768, 32767) as i16, (planned_value!("stickers", 0i64 => cantrip_mechanics::stickers::measure(q, &q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>(), StickerMeasure::TotalToughness))).clamp(-32768, 32767) as i16) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
hoisted_5.extend(objects_4.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ambassador Blorpityblorpboop", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Alien,Subtype::Advisor,Subtype::Guest]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When Ambassador Blorpityblorpboop enters, you get {TK}{TK}{TK}, then you may put a sticker on a nonland permanent you own.").targets(&[]),triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each combat, you may have Ambassador Blorpityblorpboop's base power become equal to the total power of all stickers on permanents you control and its base toughness become equal to those stickers' total toughness.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
