// Volatile Rift
// status: planned
// requires: engine: no perpetual effects
// Vivid — Whenever a creature card is put into your hand from your library, it perpetually gets +X/+0, where X is the number of colors among permanents you control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Hand) && from.is(ZoneKind::Library) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.state().obj(id).owner)) && !q.state().obj(id).token { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("engine-no-perpetual-effects" => let objects_1: Vec<ObjectId> = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() };
let kind_1_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump(({ let mut seen = ColorSet::NONE; for id in q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>() { for color in q.chars(id).colors.iter() { seen = seen.with(color); } } seen.iter().count() as i64 }).clamp(-32768, 32767) as i16, (0i64) as i16)) };
for &object in &objects_1 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_1_0); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Volatile Rift", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::G)),Some((Mana::R, Mana::G)),None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Vivid — Whenever a creature card is put into your hand from your library, it perpetually gets +X/+0, where X is the number of colors among permanents you control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
