// Tiana, Angelic Mechanic
// status: planned
// requires: engine: no crew event
// requires: engine: no perpetual effects
// Flying
// When Tiana or another legendary creature you control crews a Vehicle, that Vehicle perpetually gets +1/+0.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_1 { let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (0i64) as i16)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-crew-event" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Crewed { creature, vehicle } = event { let id = *creature; let other = *vehicle; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if Some(asker.source) == Some(id) && q.chars(other).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle)) { return Some(Fired { subject, data }); } }
if let Event::Crewed { creature, vehicle } = event { let id = *creature; let other = *vehicle; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if (!(Some(asker.source) == Some(id)) && (q.chars(id).supertypes.legendary && q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))) && q.chars(other).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle)) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Tiana, Angelic Mechanic", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("engine: no crew event", EventMask(0) => EventMask(EventMask::CREWED.0)), ability_1_event), ability_1, "When Tiana or another legendary creature you control crews a Vehicle, that Vehicle perpetually gets +1/+0.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
