// Order of Succession
// status: planned
// requires: a choice of Direction
// requires: per-player in a chosen direction
// requires: player reference InChosenDirection
// Choose left or right. Starting with you and proceeding in the chosen direction, each player chooses a creature controlled by the next player in that direction. Each player gains control of the creature they chose.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let chosen_0 = planned_value!("a choice of Direction", 0u8 => cantrip_mechanics::choose::direction(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source)?);
g.end_sentence();
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_1 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_1.iter().position(|&p| Some(p) == first_1) { players_1.rotate_left(at); }
planned!("per-player-in-a-chosen-direction" => players_1 = cantrip_mechanics::players::in_chosen_direction(g, players_1, asker)?;);
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_1.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let selected_2_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && planned_value!("player reference InChosenDirection", None::<PlayerId> => q.next_in_chosen_direction(asker, Some(bound_0))) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::ChooseAsEnters, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
 each_3[index_1] = selected_2.clone(); all_3.extend(each_3[index_1].iter().copied());
   } 
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_4.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
g.one_instruction(&mut |g| { for (index_4, bound_0) in players_4.iter().copied().enumerate() { let objects_5 = { let q: &dyn Query = g; (Some(bound_0).and_then(|who: PlayerId| players_1.iter().position(|&p| p == who)).and_then(|at| each_3[at].first().copied()).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let duration_5 = Duration::Indefinite;
let kind_5_0_0 = { let q: &dyn Query = g; EffectKind::Control(Some(bound_0).expect("a reference that always names a player")) };
if let Some(source) = source_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.add_effect(source, object, kind_5_0_0, duration_5);  } }); }
 each_6[index_4] = objects_5.clone(); all_6.extend(each_6[index_4].iter().copied());
   } Ok(()) })?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Order of Succession", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
