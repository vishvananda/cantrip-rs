// Hinterland Chef
// status: planned
// requires: engine: no perpetual effects
// requires: operation DraftFromSpellbook
// When this creature enters, draft a card from Hinterland Chef's spellbook. It perpetually becomes a Food artifact in addition to its other types and gains "{2}, {T}, Sacrifice this creature: You gain 3 life."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_4 = { let q: &dyn Query = g; (3i64).max(0) as i32 }; let players_4 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.gain_life(player, amount_4); }
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let times_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut drafted_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { drafted_0.extend(planned_value!("operation DraftFromSpellbook", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::draft_from_spellbook(g, player, asker.source, "Hinterland Chef", times_0)?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; (drafted_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddSubtype(Subtype::Food)) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); let kind_2_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddTypes(TypeSet::of(&[CardType::Artifact]))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_1); let kind_2_2 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_2); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Hinterland Chef", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, draft a card from Hinterland Chef's spellbook. It perpetually becomes a Food artifact in addition to its other types and gains \"{2}, {T}, Sacrifice this creature: You gain 3 life.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), perpetual_3, "{2}, {T}, Sacrifice this creature: You gain 3 life.").targets(&[])], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
