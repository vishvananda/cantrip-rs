// Goblin Game
// status: planned
// requires: condition ChoseExtreme
// requires: numeric expression PlayerNumber ChosenNumber
// requires: operation ChooseValueWith
// requires: operation RevealChoices
// Each player hides at least one item, then all players reveal them simultaneously. Each player loses life equal to the number of items they revealed. The player who revealed the fewest items then loses half their life, rounded up. If two or more players are tied for fewest, each loses half their life, rounded up.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let chosen_1 = planned_value!("operation ChooseValueWith", 0u32 => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Number, ChoiceTerms { options: &["Comparison(Greater, Constant(0))"], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
 each_2[index_0] = i64::from(chosen_1);
   } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Number));
g.end_sentence();
let mut players_4: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_4, bound_0) in players_4.iter().copied().enumerate() { let amount_5 = { let q: &dyn Query = g; (Some(bound_0).map_or(0i64, |p: PlayerId| planned_value!("numeric expression PlayerNumber ChosenNumber", 0i64 => q.player_number(p, PlayerNumber::ChosenNumber)))).clamp(0, 2147483647) as i32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.lose_life(player, amount_5); }
    } 
g.end_sentence();
let mut players_6: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition ChoseExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.chose_extreme(asker, p, false)))).collect::<Vec<PlayerId>>() };
 for (index_6, bound_0) in players_6.iter().copied().enumerate() { let amount_7 = { let q: &dyn Query = g; ((Some(bound_0).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))).saturating_add(1).div_euclid(2)).max(0) as i32 }; let players_7 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.lose_life(player, amount_7); }
    } 
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Goblin Game", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
