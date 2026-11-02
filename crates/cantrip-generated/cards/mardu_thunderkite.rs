// Mardu Thunderkite
// status: planned
// requires: engine: no perpetual effects
// Flying
// Mobilize 1
// When this creature enters, creatures you control perpetually gain your choice of menace, lifelink, or haste.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::choose::option(g, player_0, asker.source, item.ability.card, 0..3)? - 0;
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(match chosen_0 { 0 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Menace])), 1 => EffectKind::AddKeywords(Keywords::of(&[Keyword::Lifelink])), _ => EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])), }) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mardu Thunderkite", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::mobilize::trigger::<1>("Mobilize"),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When this creature enters, creatures you control perpetually gain your choice of menace, lifelink, or haste.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[cantrip_mechanics::mobilize::SACRIFICE], enters: None, modal: None, enchant: None, labels: &["menace", "lifelink", "haste"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::mobilize::WARRIOR];
