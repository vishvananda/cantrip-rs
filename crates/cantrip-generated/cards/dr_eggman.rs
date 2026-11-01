// Dr. Eggman
// status: planned
// requires: instruction ChooseOne
// requires: per-player instruction with several steps
// Flying
// At the beginning of your end step, draw a card. Then each opponent faces a villainous choice — That player discards a card, or you may put a Construct, Robot, or Vehicle card from your hand onto the battlefield.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_13: Vec<ObjectId> = Vec::new();
let mut hoisted_12: Vec<ObjectId> = Vec::new();
let mut hoisted_11: Vec<ObjectId> = Vec::new();
let mut hoisted_10: Vec<ObjectId> = Vec::new();
let mut hoisted_9: Vec<ObjectId> = Vec::new();
let mut hoisted_8: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
planned!("per-player-instruction-with-several-steps" => g.begin_batch(););
let mut each_14: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_14: Vec<ObjectId> = Vec::new();
let mut each_15: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_15: Vec<ObjectId> = Vec::new();
let mut each_16: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_16: Vec<ObjectId> = Vec::new();
let mut each_17: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_17: Vec<ObjectId> = Vec::new();
let mut each_18: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_18: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let decider_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; match planned_value!("instruction ChooseOne", 0u8 => cantrip_mechanics::choose::one_of(g, decider_2, asker.source, vec![Label::Card(card_2, 0), Label::Card(card_2, 1)])?) {
0 => {
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_3)?); } let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
hoisted_4.extend(discarded_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; discarded_3.clone() });
}
_ => {
{
let selected_6_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Construct)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Robot)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle)))).collect::<Vec<ObjectId>>() }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::PutOntoBattlefield, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
if !selected_6.is_empty() { 
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; selected_6.clone() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
hoisted_8.extend(moved_7.clone());
hoisted_9.extend(selected_6.clone());
hoisted_10.extend({ let q: &dyn Query = g; objects_7.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
}
g.end_sentence();
hoisted_11.extend(hoisted_8.clone());
hoisted_12.extend(hoisted_9.clone());
hoisted_13.extend({ let q: &dyn Query = g; hoisted_10.clone() });
}
}
 each_14[index_1] = hoisted_12.clone(); all_14.extend(each_14[index_1].iter().copied());
each_15[index_1] = hoisted_4.clone(); all_15.extend(each_15[index_1].iter().copied());
each_16[index_1] = hoisted_11.clone(); all_16.extend(each_16[index_1].iter().copied());
each_17[index_1] = { let q: &dyn Query = g; hoisted_5.clone() }; all_17.extend(each_17[index_1].iter().copied());
each_18[index_1] = { let q: &dyn Query = g; hoisted_13.clone() }; all_18.extend(each_18[index_1].iter().copied());
   } 
planned!("per-player-instruction-with-several-steps" => g.end_batch(););
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Dr. Eggman", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Scientist]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of your end step, draw a card. Then each opponent faces a villainous choice — That player discards a card, or you may put a Construct, Robot, or Vehicle card from your hand onto the battlefield.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
