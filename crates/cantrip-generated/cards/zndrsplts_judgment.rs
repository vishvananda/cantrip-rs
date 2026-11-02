// Zndrsplt's Judgment
// status: planned
// requires: per-player CreateTokens
// For each player, choose friend or foe. Each friend creates a token that's a copy of a creature they control. Each foe returns a creature they control to its owner's hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let card_0 = g.card_id(&CARD); let mut words_0: Vec<(PlayerId, u8)> = Vec::new(); for player in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } { let word = cantrip_mechanics::choose::option(g, asker.controller, asker.source, card_0, 0..2)?; words_0.push((player, word)); }
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 0)).collect::<Vec<PlayerId>>() };
planned!("per-player-create-tokens" => g.begin_batch(););
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let player_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let among_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let chosen_2 = cantrip_mechanics::choose::select(g, player_2, asker.source, Purpose::CopyWhich, among_2, cantrip_mechanics::select::cards(cantrip_mechanics::select::exactly(1)))?.first().copied();
let values_2 = { let q: &dyn Query = g; chosen_2.map(|original: ObjectId| q.copiable_values(original)) }; let count_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let created_2 = match values_2 { Some(values) => g.create_token_copies_tapped(values, player_2, count_2, false)?, None => Vec::new() };
 each_3[index_1] = created_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
planned!("per-player-create-tokens" => g.end_batch(););
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| words_0.iter().any(|&(p, word)| Some(p) == Some(bound_0) && word == 1)).collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let selected_5_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_5_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_5_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_5 = { cantrip_mechanics::choose::select(g, selected_5_player, asker.source, Purpose::PutIntoHand, selected_5_candidates, cantrip_mechanics::select::Selection::of(selected_5_count, &[]))? };
 each_6[index_4] = selected_5.clone(); all_6.extend(each_6[index_4].iter().copied());
   } 
g.end_sentence();
let objects_7 = { let q: &dyn Query = g; all_6.clone() }; let moved_7 = g.return_to_hand(&objects_7);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Zndrsplt's Judgment", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Friend", "Foe"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
