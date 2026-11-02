// Seize the Spotlight
// status: planned
// requires: condition VotedFor
// requires: per-player GiveControl
// Each opponent chooses fame or fortune. For each player who chose fame, gain control of a creature that player controls until end of turn. Untap those creatures and they gain haste until end of turn. For each player who chose fortune, you draw a card and create a Treasure token.
#[allow(unused_imports)] use cantrip_api::*;
pub mod token_12 {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: token!(Treasure).name, token_words: token!(Treasure).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Treasure]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(SACRIFICE_THIS).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "Add mana.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_0.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let card_1 = g.card_id(&CARD); let chosen_1: u8 = cantrip_mechanics::choose::option(g, player_1, asker.source, card_1, 0..2)?;
 each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
let mut players_2: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>() };
planned!("per-player-give-control" => g.begin_batch(););
let mut each_7: Vec<Vec<ObjectId>> = vec![Vec::new(); players_2.len()]; let mut all_7: Vec<ObjectId> = Vec::new();
 for (index_2, bound_9000) in players_2.iter().copied().enumerate() { let selected_4_player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let selected_4_candidates = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| Some(bound_9000) == Some(q.controller(id))).collect::<Vec<ObjectId>>() }; let selected_4_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; let selected_4 = { cantrip_mechanics::choose::select(g, selected_4_player, asker.source, Purpose::Choose, selected_4_candidates, cantrip_mechanics::select::Selection::of(selected_4_count, &[]))? };
let players_5: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let objects_5 = { let q: &dyn Query = g; selected_4.clone() }; let who_5 = match players_5.as_slice() { [one] => Some(*one), _ => cantrip_mechanics::choose::player(g, asker.controller, asker.source, players_5.clone(), "GainControl", 0)? }; if let Some(who) = who_5 { cantrip_api::instruction(g, |g| { for &object in &objects_5 { g.gain_control(asker.source, object, who, Duration::EndOfTurn); } }); }
 each_7[index_2] = selected_4.clone(); all_7.extend(each_7[index_2].iter().copied());
   } 
planned!("per-player-give-control" => g.end_batch(););
g.end_sentence();
let objects_8 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1)).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_8: Vec<ObjectId> = objects_8.iter().copied().filter(|&o| g.state().obj(o).tapped != false).collect(); cantrip_api::instruction(g, |g| { for &object in &turned_8 { g.untap(object); } });
g.end_sentence();
let objects_9 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_1)).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let duration_9 = Duration::EndOfTurn;
let kind_9_0_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::Haste])) };
if let Some(source) = source_9 { cantrip_api::instruction(g, |g| { for &object in &objects_9 { g.add_effect(source, object, kind_9_0_0, duration_9);  } }); }
g.end_sentence();
let amount_10 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let players_10 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_10 { g.draw(player, amount_10); }
g.end_sentence();
let players_11 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_11 = Vec::new();
let count_11_0 = { let q: &dyn Query = g; ((q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition VotedFor", false => g.votes().voted(option))).collect::<Vec<PlayerId>>()).len() as i64).max(0) as u32 }; let def_11_0 = g.card_id(&token_12::CARD);
for player in players_11 {
tokens_11.extend(std::iter::repeat((def_11_0,player)).take(count_11_0 as usize));
} let words_11 = Words::NONE;
let created_11 = g.create_tokens_worded(&tokens_11, false, None, words_11)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Seize the Spotlight", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, labels: &["Fame", "Fortune"], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_12::CARD];
