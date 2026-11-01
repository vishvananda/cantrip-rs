// Skyline Savior
// status: planned
// requires: engine: no perpetual effects
// Flying
// When this creature enters, return a permanent you control to its owner's hand. If it's a non-Angel creature card, it perpetually gets +1/+1, gains flying, and becomes an Angel in addition to its other types.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_0_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_0_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| Some(asker.controller) == Some(q.controller(id))).collect::<Vec<ObjectId>>() }; let selected_0_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_0 = { cantrip_mechanics::choose::select(g, selected_0_player, asker.source, Purpose::PutIntoHand, selected_0_candidates, cantrip_mechanics::select::Selection::of(selected_0_count, &[]))? };
let objects_1 = { let q: &dyn Query = g; selected_0.clone() }; let moved_1 = g.return_to_hand(&objects_1);
g.end_sentence();
if { let q: &dyn Query = g; (((moved_1.clone()).into_iter().filter(|&id| (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Angel))) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>()).len() as i64) > (0i64) } {
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; (moved_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::Pump((1i64) as i16, (1i64) as i16)) };
let kind_3_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Flying]))) };
let kind_3_2 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Angel))) };
for &object in &objects_3 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_1); cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_2); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Skyline Savior", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "When this creature enters, return a permanent you control to its owner's hand. If it's a non-Angel creature card, it perpetually gets +1/+1, gains flying, and becomes an Angel in addition to its other types.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
