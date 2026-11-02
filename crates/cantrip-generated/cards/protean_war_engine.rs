// Protean War Engine
// status: planned
// requires: operation DraftFromSpellbook
// requires: trigger event Object Crewed
// As this Vehicle enters, draft a card from Protean War Engine's spellbook and exile it.
// Whenever this Vehicle becomes crewed, until end of turn, it becomes a copy of the exiled card, except it's a Vehicle artifact in addition to its other types.
// Crew 3
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let times_0: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut drafted_0: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() } { drafted_0.extend(planned_value!("operation DraftFromSpellbook", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::draft_from_spellbook(g, player, asker.source, "Protean War Engine", times_0)?)); }
let objects_1 = { let q: &dyn Query = g; (drafted_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let moved_1 = { let by = g.text_linked(asker.source); g.exile_with(&objects_1, by, false) };
 Ok(()) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let original_2 = { let q: &dyn Query = g; (((q.state().seats().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::exile(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && q.exiled_with(q.linked_to(asker)).contains(&id)).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }; let values_2 = { let q: &dyn Query = g; original_2.first().copied().map(|original: ObjectId| q.copiable_values(original).with_subtype(q.words(asker.source).subtype(Subtype::Vehicle)).with_type(CardType::Artifact)) }; if let (Some(source), Some(values)) = (source_2, values_2) { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.become_copy(source, object, values, duration_2); } }); }
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-object-crewed" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Crewed { vehicle, .. } = event { let id = *vehicle; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Protean War Engine", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Object Crewed", EventMask(0) => EventMask(EventMask::CREWED.0)), ability_1_event), ability_1, "Whenever this Vehicle becomes crewed, until end of turn, it becomes a copy of the exiled card, except it's a Vehicle artifact in addition to its other types.").targets(&[]).once_per_batch(),cantrip_mechanics::crew::crew(3, "Crew 3")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
