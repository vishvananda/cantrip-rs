// Caught in a Parallel Universe
// status: planned
// requires: card type Phenomenon
// requires: layout Planar
// requires: trigger event Player EncountersPhenomenon
// When you encounter Caught in a Parallel Universe, each player chooses a creature controlled by the player to their left. Each player creates a token that's a copy of the creature they chose, except it has menace. (Then planeswalk away from this phenomenon.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_1_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0).and_then(|p: PlayerId| q.state().turn_order_from(p).filter(|&next| next != p).next()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::ChooseAsEnters, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
 each_2[index_0] = selected_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_4: Vec<(Copiable, PlayerId)> = Vec::new();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { let player_5 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let originals_5 = { let q: &dyn Query = g; Some(bound_0).and_then(|who: PlayerId| players_0.iter().position(|&p| p == who)).and_then(|at| each_2[at].first().copied()) }.into_iter().collect::<Vec<ObjectId>>(); for original in originals_5 { if let Some(values) = { let q: &dyn Query = g; Some(original).map(|original: ObjectId| q.copiable_values(original).with_keyword(Keyword::Menace)) } { gathered_4.push((values, player_5)); } }
    } 
let created_4 = g.create_token_copies_together(&gathered_4)?;
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-player-encounters-phenomenon" => matcher: #[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::EncounteredPhenomenon { player, object, .. } = event { let id = *object; let subject = Target::Object(id); let data = [u32::from(player.0), 0]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });
planned_item!("layout-planar" => variant: Some(Variant::Planechase));

pub static CARD: CardDef = CardDef { name: "Caught in a Parallel Universe", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Phenomenon", CardType::Land => CardType::Phenomenon)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Player EncountersPhenomenon", EventMask(0) => EventMask(EventMask::PHENOMENON.0)), ability_0_event), ability_0, "When you encounter Caught in a Parallel Universe, each player chooses a creature controlled by the player to their left. Each player creates a token that's a copy of the creature they chose, except it has menace.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
