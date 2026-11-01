// Rebirth
// status: planned
// requires: card outside ordinary play
// requires: move to Ante
// Remove this card from your deck before playing if you're not playing for ante.
// Each player may ante the top card of their library. If a player does, that player's life total becomes 20.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
let mut each_10 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let payer_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let paid_1 = cantrip_mechanics::effects::ask_may(g, payer_1, asker.source, Purpose::Optional)?; if paid_1 {
let objects_3 = { let q: &dyn Query = g; (((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_3 = planned_value!("move to Ante", Vec::<ObjectId>::new() => g.ante(&objects_3));
g.end_sentence();
hoisted_4.extend(moved_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; objects_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
 each_6[index_0] = hoisted_4.clone(); all_6.extend(each_6[index_0].iter().copied());
each_7[index_0] = hoisted_4.clone(); all_7.extend(each_7[index_0].iter().copied());
each_8[index_0] = { let q: &dyn Query = g; hoisted_4.clone() }; all_8.extend(each_8[index_0].iter().copied());
each_9[index_0] = { let q: &dyn Query = g; hoisted_5.clone() }; all_9.extend(each_9[index_0].iter().copied());
each_10[index_0] = paid_1;
   } 
g.end_sentence();
let mut players_11: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| Some(bound_0).and_then(|p: PlayerId| players_0.iter().position(|&next| next == p)).is_some_and(|at| (each_10[at] as i64) != 0)).collect::<Vec<PlayerId>>() };
 for (index_11, bound_0) in players_11.iter().copied().enumerate() { let amount_12 = { let q: &dyn Query = g; (20i64).max(0) as i32 }; let players_12 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; for player in players_12 { g.set_life_total(player, amount_12); }
    } 
g.end_sentence();
 Ok(()) }
planned_item!("card-outside-ordinary-play" => deck_rules: DeckRule::OnlyWhenPlayingForAnte);

pub static CARD: CardDef = CardDef { name: "Rebirth", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
