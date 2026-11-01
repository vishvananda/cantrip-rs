// Darksteel Hydra
// status: planned
// requires: operation Conjure
// Indestructible
// This creature enters with X oil counters on it.
// Darksteel Hydra's power and toughness are each equal to twice the number of oil counters on it.
// When this creature enters, conjure a card named Darksteel Ingot and a card named Darksteel Plate into your hand.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (i64::from(entering.x)).clamp(0, 32767) as i16 }; entering.counters[CounterKind::Oil as usize] = entering.counters[CounterKind::Oil as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = (((2i64 * (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Oil))).sum::<i64>())).clamp(-32768, 32767) as i16, ((2i64 * (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Oil))).sum::<i64>())).clamp(-32768, 32767) as i16); chars.power = Some(p); chars.toughness = Some(t); }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Darksteel Ingot"), count_1, ConjureTo::Hand)?)); }
g.end_sentence();
let count_2: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_2: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_2.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::Named("Darksteel Plate"), count_2, ConjureTo::Hand)?)); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Darksteel Hydra", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 1, symbols: [1, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Hydra]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Indestructible]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When this creature enters, conjure a card named Darksteel Ingot and a card named Darksteel Plate into your hand.").targets(&[])], statics: &[characteristic_defining(Layer::PtDefine, ability_2_apply_0, "Darksteel Hydra's power and toughness are each equal to twice the number of oil counters on it.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
