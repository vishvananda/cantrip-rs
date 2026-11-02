// Tomb of Annihilation
// status: planned
// requires: ability DungeonRoom
// requires: layout Dungeon
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (1i64).max(0) as i32 }; let players_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
for player in players_0 { g.lose_life(player, amount_0); }
g.end_sentence();
 Ok(()) }
fn ability_1_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_1_0 = vec![false; players_1.len()];
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
let mut each_7 = vec![Default::default(); players_1.len()];
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let payer_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_2, after_2, before_2, picked_2): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let amount_4 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_4: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_4 = Vec::new(); for player in players_4 { cards_4.extend({ let hand = g.state().zone(Zone::hand(player)).to_vec(); cantrip_mechanics::choose::cards(g, player, asker.source, Purpose::Discard, hand, amount_4, true)? }); } let discarded_4 = g.discard_together(&cards_4);
let paid_3 = !discarded_4.clone().is_empty();
(paid_3, { let q: &dyn Query = g; (discarded_4).clone() }, { let q: &dyn Query = g; (discarded_4.clone()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_5[index_1] = after_2.clone(); all_5.extend(each_5[index_1].iter().copied());
each_6[index_1] = { let q: &dyn Query = g; before_2.clone() }; all_6.extend(each_6[index_1].iter().copied());
each_7[index_1] = paid_2;
   } 
g.end_sentence();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_7[index_1] as i64) != 0)) } { guard_1_0[index_1] = true;
let amount_8 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_8 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_8 { g.lose_life(player, amount_8); }
   }
 } 
g.end_sentence();
 Ok(()) }
fn ability_2_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_9: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut guard_9_0 = vec![false; players_9.len()];
let mut each_12: Vec<Vec<ObjectId>> = vec![Vec::new(); players_9.len()]; let mut all_12: Vec<ObjectId> = Vec::new();
let mut each_13: Vec<Vec<ObjectId>> = vec![Vec::new(); players_9.len()]; let mut all_13: Vec<ObjectId> = Vec::new();
let mut each_14 = vec![Default::default(); players_9.len()];
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { let payer_10 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let (paid_10, after_10, before_10, picked_10): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if true {
let objects_11 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Land))).collect::<Vec<ObjectId>>() }; let players_11 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }; let mut picks_11: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_11 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::UpTo((1i64).max(0) as u32) }; for player in players_11 { let eligible: Vec<ObjectId> = objects_11.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_11.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() < n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_11.clone(), &[]))? }; for object in taken { picks_11.push((player, object)); } }
let sacrificed_11 = g.sacrifice_each(&picks_11);
let paid_11 = !picks_11.is_empty();
(paid_11, { let q: &dyn Query = g; (sacrificed_11).clone() }, { let q: &dyn Query = g; (objects_11.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>()).clone() }, Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
 each_12[index_9] = after_10.clone(); all_12.extend(each_12[index_9].iter().copied());
each_13[index_9] = { let q: &dyn Query = g; before_10.clone() }; all_13.extend(each_13[index_9].iter().copied());
each_14[index_9] = paid_10;
   } 
g.end_sentence();
 for (index_9, bound_0) in players_9.iter().copied().enumerate() { if { let q: &dyn Query = g; !(((each_14[index_9] as i64) != 0)) } { guard_9_0[index_9] = true;
let amount_15 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_15 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_15 { g.lose_life(player, amount_15); }
   }
 } 
g.end_sentence();
 Ok(()) }
fn ability_3_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_16 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_16: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_16 = Vec::new(); for player in players_16 { cards_16.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_16)?); } let discarded_16 = g.discard_together(&cards_16);
g.end_sentence();
let player_17: PlayerId = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let mut slots_17: Vec<Vec<ObjectId>> = Vec::new();
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; slots_17.push(slot.into_iter().filter(|&o| g.controller(o) == player_17 && g.can_sacrifice(player_17, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }; slots_17.push(slot.into_iter().filter(|&o| g.controller(o) == player_17 && g.can_sacrifice(player_17, o)).collect());
let slot: Vec<ObjectId> = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Land)).collect::<Vec<ObjectId>>() }; slots_17.push(slot.into_iter().filter(|&o| g.controller(o) == player_17 && g.can_sacrifice(player_17, o)).collect());
let mut chosen_17: Vec<ObjectId> = Vec::new(); for at in 0..slots_17.len() { let goal = most_assigned(&slots_17[at..], &mut chosen_17.clone()); let keeps: Vec<ObjectId> = slots_17[at].iter().copied().filter(|o| !chosen_17.contains(o)).filter(|&o| { let mut taken = chosen_17.clone(); taken.push(o); 1 + most_assigned(&slots_17[at + 1..], &mut taken) == goal }).collect(); let picked = match keeps.as_slice() { [] => None, [one] => Some(*one), _ => cantrip_mechanics::choose::card(g, player_17, asker.source, Purpose::Sacrifice, keeps.clone())? }; if let Some(o) = picked { chosen_17.push(o); } }
let sacrificed_17 = g.sacrifice(player_17, &chosen_17);
g.end_sentence();
 Ok(()) }
pub mod token_19 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: "The Atropal", token_words: &[], oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::God,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: Some(ColorSet(4)), power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Deathtouch]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_4_room(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_18 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_18 = Vec::new();
let count_18_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_18_0 = g.card_id(&token_19::CARD);
for player in players_18 {
tokens_18.extend(std::iter::repeat((def_18_0,player)).take(count_18_0 as usize));
} let words_18 = g.words(asker.source);
let created_18 = g.create_tokens_worded(&tokens_18, false, None, words_18)?;
g.end_sentence();
 Ok(()) }
fn most_assigned(slots: &[Vec<ObjectId>], taken: &mut Vec<ObjectId>) -> usize {
    let Some((first, rest)) = slots.split_first() else { return 0 };
    let mut best = most_assigned(rest, taken);
    for &object in first {
        if !taken.contains(&object) {
            taken.push(object);
            best = best.max(1 + most_assigned(rest, taken));
            taken.pop();
        }
    }
    best
}
planned_item!("ability-dungeon-room" => rooms: Room { name: "Trapped Entry", effect: spell(&[], ability_0_room), leads_to: &["Veils of Fear", "Oubliette"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Veils of Fear", effect: spell(&[], ability_1_room), leads_to: &["Sandfall Cell"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Sandfall Cell", effect: spell(&[], ability_2_room), leads_to: &["Cradle of the Death God"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Oubliette", effect: spell(&[], ability_3_room), leads_to: &["Cradle of the Death God"] });
planned_item!("ability-dungeon-room" => rooms: Room { name: "Cradle of the Death God", effect: spell(&[], ability_4_room), leads_to: &[] });
planned_item!("layout-dungeon" => dungeon: &[&[1, 3], &[2], &[4], &[4], &[]]);

pub static CARD: CardDef = CardDef { name: "Tomb of Annihilation", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_19::CARD];
