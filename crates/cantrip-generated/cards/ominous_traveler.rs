// Ominous Traveler
// status: planned
// requires: engine: no perpetual effects
// requires: operation DraftFromSpellbook
// When this creature enters, draft a card from Ominous Traveler's spellbook. That card perpetually gains "Mana of any type can be spent to cast this spell" and "When you cast this spell, return a creature named Ominous Traveler you control to its owner's hand."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
#[allow(unused_variables)] fn perpetual_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn perpetual_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let selected_4_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.chars(id).name == "Ominous Traveler" && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::PutIntoHand, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
let objects_5 = { let q: &dyn Query = g; selected_4.clone() }; let moved_5 = g.return_to_hand(&objects_5);
g.end_sentence();
 Ok(()) }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let times_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut drafted_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { drafted_0.extend(planned_value!("operation DraftFromSpellbook", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::draft_from_spellbook(g, player, asker.source, "Ominous Traveler", times_0)?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; (drafted_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::SpendAsThoughAnyType }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); let kind_2_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddAbility(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0))) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_1); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Ominous Traveler", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "When this creature enters, draft a card from Ominous Traveler's spellbook. That card perpetually gains \"Mana of any type can be spent to cast this spell\" and \"When you cast this spell, return a creature named Ominous Traveler you control to its owner's hand.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), perpetual_3_event), perpetual_3, "When you cast this spell, return a creature named Ominous Traveler you control to its owner's hand.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack]))], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
