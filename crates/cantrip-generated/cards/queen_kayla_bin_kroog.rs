// Queen Kayla bin-Kroog
// status: ready
// {4}, {T}: Discard all the cards in your hand, then draw that many cards. You may choose an artifact or creature card with mana value 1 you discarded this way, then do the same for artifact or creature cards with mana values 2 and 3. Return those cards to the battlefield. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_0 = Vec::new(); for player in players_0 { cards_0.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_0)?); } let discarded_0 = g.discard_together(&cards_0);
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; ((discarded_0.clone()).len() as i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.draw(player, amount_1); }
g.end_sentence();
{
let selected_2_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; (discarded_0.clone()).into_iter().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (i64::from(q.mana_value(id))) == (1i64))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::ChooseAsEnters, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
if !selected_2.is_empty() { 
g.end_sentence();
hoisted_3.extend(selected_2.clone());
}
}
g.end_sentence();
{
let selected_4_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; (discarded_0.clone()).into_iter().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (i64::from(q.mana_value(id))) == (2i64))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::ChooseAsEnters, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
if !selected_4.is_empty() { 
g.end_sentence();
hoisted_5.extend(selected_4.clone());
}
}
g.end_sentence();
{
let selected_6_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_6_candidates = { let q: &dyn Query = g; (discarded_0.clone()).into_iter().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (i64::from(q.mana_value(id))) == (3i64))).collect::<Vec<ObjectId>>() }; let selected_6_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; let selected_6 = { cantrip_mechanics::choose::select(g, selected_6_player, asker.source, Purpose::ChooseAsEnters, selected_6_candidates, cantrip_mechanics::select::Selection::of(selected_6_count, &[]))? };
if !selected_6.is_empty() { 
g.end_sentence();
hoisted_7.extend(selected_6.clone());
}
}
g.end_sentence();
let player_8 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_8 = ({ let q: &dyn Query = g; { let mut all: Vec<ObjectId> = Vec::new(); for id in hoisted_3.clone() { if !all.contains(&id) { all.push(id); } } for id in hoisted_5.clone() { if !all.contains(&id) { all.push(id); } } for id in hoisted_7.clone() { if !all.contains(&id) { all.push(id); } } all } }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_8 = { let rows_8: Vec<(ObjectId, PlayerId, bool)> = objects_8.iter().map(|&o| (o, player_8, false)).collect(); let came_8 = g.put_all_onto_battlefield_as(&rows_8)?; came_8.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Queen Kayla bin-Kroog", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Noble]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{4}, {T}: Discard all the cards in your hand, then draw that many cards. You may choose an artifact or creature card with mana value 1 you discarded this way, then do the same for artifact or creature cards with mana values 2 and 3. Return those cards to the battlefield. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
