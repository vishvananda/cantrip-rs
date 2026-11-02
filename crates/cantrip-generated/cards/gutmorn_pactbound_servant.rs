// Gutmorn, Pactbound Servant
// status: planned
// requires: engine: no perpetual effects
// requires: operation Conjure
// Flying, Deathtouch
// When Gutmorn enters, each player discards a nonland card.
// Whenever a player discards a card during your turn, they choose another player. That player conjures a duplicate of that card into their hand. It perpetually gains "Mana of any type can be spent to cast this spell."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_2: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_1_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_1_candidates = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && !(q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let selected_1_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_1 = { cantrip_mechanics::choose::select(g, selected_1_player, asker.source, Purpose::Discard, selected_1_candidates, cantrip_mechanics::select::Selection::of(selected_1_count, &[]))? };
 each_2[index_0] = selected_1.clone(); all_2.extend(each_2[index_0].iter().copied());
   } 
g.end_sentence();
let cards_3 = { let q: &dyn Query = g; all_2.clone() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if true && true && !*undefined && Some(asker.controller) == Some(q.state().turn.active) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_4 = { let q: &dyn Query = g; Some(PlayerId(item.data[0] as u8)).expect("a reference that always names a player") }; let candidates_4 = { let q: &dyn Query = g; { let other = Some(PlayerId(item.data[0] as u8)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } }; let Some(chosen_4) = cantrip_mechanics::choose::player(g, player_4, asker.source, candidates_4, "Choose", 0)? else { return Ok(()) };
g.end_sentence();
let duplicated_5 = { let q: &dyn Query = g; ((item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() };
let count_5: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_5: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(chosen_4).into_iter().collect::<Vec<PlayerId>>() } { conjured_5.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_5), count_5, ConjureTo::Hand)?)); }
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_7: Vec<ObjectId> = { let q: &dyn Query = g; (conjured_5.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
for &object in &objects_7 { let kind_7_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::SpendAsThoughAnyType }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_7_0); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Gutmorn, Pactbound Servant", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "When Gutmorn enters, each player discards a nonland card.").targets(&[]),triggered(when_asked(EventMask(EventMask::DISCARDED.0), ability_3_event), ability_3, "Whenever a player discards a card during your turn, they choose another player. That player conjures a duplicate of that card into their hand. It perpetually gains \"Mana of any type can be spent to cast this spell.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
