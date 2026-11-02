// Order of Succession
// status: ready
// Choose left or right. Starting with you and proceeding in the chosen direction, each player chooses a creature controlled by the next player in that direction. Each player gains control of the creature they chose.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let card_0 = g.card_id(&CARD); let chosen_0: u8 = cantrip_mechanics::choose::option(g, player_0, asker.source, card_0, 0..2)?;
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_1 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_1.iter().position(|&p| Some(p) == first_1) { players_1.rotate_left(at); }
if chosen_0 == 1 && players_1.len() > 1 { players_1[1..].reverse(); }
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let selected_2_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(bound_0).and_then(|p: PlayerId| { let order: Vec<PlayerId> = q.state().turn_order_from(p).collect(); if order.len() < 2 { None } else if chosen_0 == 0 { Some(order[1]) } else { order.last().copied() } }) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::ChooseAsEnters, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
 each_3[index_1] = selected_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let snap_5: Vec<Vec<ObjectId>> = players_4.iter().copied().map(|bound_0| { let q: &dyn Query = g; Some(bound_0).and_then(|who: PlayerId| players_1.iter().position(|&p| p == who)).and_then(|at| each_3[at].first().copied()).into_iter().collect::<Vec<ObjectId>>() }).collect();
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
g.one_instruction(&mut |g| { for (index_4, bound_0) in players_4.iter().copied().enumerate() { let objects_6 = { let q: &dyn Query = g; (snap_5[index_4].clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_6 = { let q: &dyn Query = g; Some(asker.source) }; let duration_6 = Duration::Indefinite;
let kind_6_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(bound_0).expect("a reference that always names a player")) };
if let Some(source) = source_6 { cantrip_api::instruction(g, |g| { for &object in &objects_6 { g.add_effect(source, object, kind_6_0_0, duration_6);  } }); }
 each_7[index_4] = objects_6.clone(); all_7.extend(each_7[index_4].iter().copied());
   } Ok(()) })?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Order of Succession", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["left", "right"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
