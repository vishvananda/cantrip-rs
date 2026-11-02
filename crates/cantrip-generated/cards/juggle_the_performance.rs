// Juggle the Performance
// status: planned
// requires: earlier objects Conjured
// requires: engine: no perpetual effects
// requires: operation Conjure
// Each player discards their hand, then conjures a duplicate of each of seven random cards from the library of the player to their right into their hand. The duplicates perpetually gain "Mana of any type can be spent to cast this spell."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { gathered_1.extend(g.state().zone(Zone::hand(bound_0)).to_vec());
    } 
let cards_3 = { let q: &dyn Query = g; gathered_1.clone() }; let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
let mut each_8: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_8: Vec<ObjectId> = Vec::new();
let mut each_9: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_9: Vec<ObjectId> = Vec::new();
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let mut pool_5: Vec<ObjectId> = { let q: &dyn Query = g; ((Some(bound_0).and_then(|p: PlayerId| q.state().turn_order_from(p).filter(|&next| next != p).last()).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() };
let count_5 = { let q: &dyn Query = g; 7i64 };
let mut random_5: Vec<ObjectId> = Vec::new(); for _ in 0..count_5 { let Some(one) = cantrip_mechanics::library::random_of(g, &pool_5) else { break }; pool_5.retain(|&o| o != one); random_5.push(one); }
let duplicated_6 = { let q: &dyn Query = g; random_5.clone() };
let count_6: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_6: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { conjured_6.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_6), count_6, ConjureTo::Hand)?)); }
 each_7[index_4] = random_5.clone(); all_7.extend(each_7[index_4].iter().copied());
each_8[index_4] = conjured_6.clone(); all_8.extend(each_8[index_4].iter().copied());
each_9[index_4] = conjured_6.clone(); all_9.extend(each_9[index_4].iter().copied());
   } 
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_11: Vec<ObjectId> = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && (planned_value!("earlier objects Conjured", Vec::<ObjectId>::new() => q.acted_on(asker, Acted::Conjured))).contains(&id)).collect::<Vec<ObjectId>>() };
for &object in &objects_11 { let kind_11_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::SpendAsThoughAnyType }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_11_0); });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Juggle the Performance", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
