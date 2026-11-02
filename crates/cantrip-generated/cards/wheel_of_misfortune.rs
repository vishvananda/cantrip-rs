// Wheel of Misfortune
// status: planned
// requires: a value chosen in secret
// requires: condition ChoseExtreme
// requires: numeric expression PlayerNumber ChosenNumber
// requires: operation RevealChoices
// Each player secretly chooses a number 0 or greater, then all players reveal those numbers simultaneously and determine the highest and lowest numbers revealed this way. Wheel of Misfortune deals damage equal to the highest number to each player who chose that number. Each player who didn't choose the lowest number discards their hand, then draws seven cards.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut each_2 = vec![Default::default(); players_0.len()];
let mut each_chosen_1 = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let chosen_1 = planned_value!("a value chosen in secret", 0u32 => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Number, ChoiceTerms { options: &[], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
 each_2[index_0] = i64::from(chosen_1);
each_chosen_1.push(chosen_1);
   } 
g.end_sentence();
planned!("operation-reveal-choices" => cantrip_mechanics::secret::reveal_choices(g, asker.source, ChoiceKind::Number));
g.end_sentence();
let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let amount_4 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| planned_value!("numeric expression PlayerNumber ChosenNumber", 0i64 => q.player_number(p, PlayerNumber::ChosenNumber))).max().unwrap_or(0)).clamp(0, 65535) as u16 }; let recipients_4 = { let q: &dyn Query = g; (q.state().apnap().filter(|&bound_0: &PlayerId| planned_value!("condition ChoseExtreme", false => Some(bound_0).is_some_and(|p: PlayerId| q.chose_extreme(asker, p, true)))).collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_4 { let batch: Vec<(ObjectId, Target, u16)> = recipients_4.iter().map(|t| (source, *t, amount_4)).collect(); g.damage_together(&batch); }
g.end_sentence();
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).is_some_and(|p: PlayerId| (q.state().apnap().filter(|&bound_1: &PlayerId| planned_value!("condition ChoseExtreme", false => Some(bound_1).is_some_and(|p: PlayerId| q.chose_extreme(asker, p, false)))).collect::<Vec<PlayerId>>()).contains(&p)))).collect::<Vec<PlayerId>>() };
let mut gathered_6: Vec<ObjectId> = Vec::new();
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { gathered_6.extend(g.state().zone(Zone::hand(bound_0)).to_vec());
    } 
let cards_8 = { let q: &dyn Query = g; gathered_6.clone() }; let discarded_8 = g.discard_together(&cards_8);
g.end_sentence();
let amount_9 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_9 = { let q: &dyn Query = g; q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).is_some_and(|p: PlayerId| (q.state().apnap().filter(|&bound_1: &PlayerId| planned_value!("condition ChoseExtreme", false => Some(bound_1).is_some_and(|p: PlayerId| q.chose_extreme(asker, p, false)))).collect::<Vec<PlayerId>>()).contains(&p)))).collect::<Vec<PlayerId>>() };
for player in players_9 { g.draw(player, amount_9); }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Wheel of Misfortune", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
