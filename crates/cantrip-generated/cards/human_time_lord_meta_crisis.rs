// Human-Time Lord Meta-Crisis
// status: planned
// requires: bounded resolution choice
// requires: card type Phenomenon
// requires: condition ChoseNth
// requires: layout Planar
// requires: object collection Earlier
// requires: object reference NthChosen
// requires: trigger event Player EncountersPhenomenon
// When you encounter Human-Time Lord Meta-Crisis, each player chooses one or two creatures they control. Each player creates a token that's a copy of the first creature they chose, except it isn't legendary. Then each player who chose a second creature puts a number of +1/+1 counters on the token they created equal to the power of the second creature they chose. (Then planeswalk away from this phenomenon.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_1_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; planned_value!("bounded resolution choice", cantrip_mechanics::select::Count::UpTo(({ let q: &dyn Query = g; 2i64 }).max(0) as u32) => cantrip_mechanics::select::Count::Between(({ let q: &dyn Query = g; 1i64 }).max(0) as u32, ({ let q: &dyn Query = g; 2i64 }).max(0) as u32)) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Choose, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
 each_2[index_0] = selected_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition ChoseNth", false => Some(bound_0).is_some_and(|p: PlayerId| q.chose_nth(asker, p, 1)))).collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<(Copiable, PlayerId)> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let player_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let originals_5 = { let q: &dyn Query = g; planned_value!("object reference NthChosen", None::<ObjectId> => q.nth_chosen(asker, 0, Some(bound_0))) }.into_iter().collect::<Vec<ObjectId>>(); for original in originals_5 { if let Some(values) = { let q: &dyn Query = g; Some(original).map(|original: ObjectId| q.copiable_values(original).nonlegendary()) } { gathered_4.push((values, player_5)); } }
    } 
let created_4 = g.create_token_copies_together(&gathered_4)?;
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition ChoseNth", false => Some(bound_0).is_some_and(|p: PlayerId| q.chose_nth(asker, p, 2)))).collect::<Vec<PlayerId>>() };
let mut gathered_7: Vec<(PlayerId, ObjectId, CounterKind, i16)> = Vec::new();
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { let mut counters_9: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (planned_value!("object reference NthChosen", None::<ObjectId> => q.nth_chosen(asker, 1, Some(bound_0))).map_or(0i64, |o: ObjectId| i64::from(q.chars(o).power.unwrap_or(0)))).max(0) as i16 }; for t in { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (planned_value!("object collection Earlier", Vec::<ObjectId>::new() => q.earlier_objects(item, EarlierObjects::Created, Some(bound_0).into_iter().collect::<Vec<PlayerId>>()))).contains(&id)).collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_9.push((object, CounterKind::PlusOne, amount)); } }
gathered_7.extend(counters_9.iter().map(|c| (bound_0, c.0, c.1, c.2)));
    } 
let had_7: Vec<_> = gathered_7.iter().map(|c| g.state().obj(c.1).counters(c.2)).collect(); g.add_counters_together_by(&gathered_7); let applied_7: Vec<ObjectId> = gathered_7.iter().zip(had_7).filter(|(c, had)| g.state().obj(c.1).counters(c.2) > *had).map(|(c, _)| c.1).collect();
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-encounters-phenomenon" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::EncounteredPhenomenon { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Human-Time Lord Meta-Crisis", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Phenomenon", CardType::Land => CardType::Phenomenon)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player EncountersPhenomenon", EventMask(0) => EventMask(EventMask::PHENOMENON.0)), ability_0_event), ability_0, "When you encounter Human-Time Lord Meta-Crisis, each player chooses one or two creatures they control. Each player creates a token that's a copy of the first creature they chose, except it isn't legendary. Then each player who chose a second creature puts a number of +1/+1 counters on the token they created equal to the power of the second creature they chose.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
