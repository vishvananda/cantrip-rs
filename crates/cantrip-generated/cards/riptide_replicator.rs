// Riptide Replicator
// status: planned
// requires: a choice of Color outside its body
// requires: a choice of CreatureType outside its body
// requires: condition ValueChosen
// requires: token of the chosen color
// As this artifact enters, choose a color and a creature type.
// This artifact enters with X charge counters on it.
// {4}, {T}: Create an X/X creature token of the chosen color and type, where X is the number of charge counters on this artifact.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Colors(colors_0) = g.ask(player_0, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_0 = colors_0[0];
entering.chosen_color = Some(chosen_0);
let Some(chosen_1) = cantrip_mechanics::chosen_type::choose_creature_type(g, entering)? else { return Ok(()) };
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (i64::from(entering.x)).clamp(0, 32767) as i16 }; entering.counters[CounterKind::Charge as usize] = entering.counters[CounterKind::Charge as usize].saturating_add(more);
 Ok(()) }
pub mod token_4 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!().name, token_words: token!().token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (planned_value!("condition ValueChosen", false => q.state().chosen_value(q.linked(item), ChoiceKind::Color).is_some()) && planned_value!("condition ValueChosen", false => q.state().chosen_value(q.linked(item), ChoiceKind::CreatureType).is_some())) } {
let created_3: Vec<ObjectId> = planned_value!("token of the chosen color", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::Filled(g.card_id(&token_4::CARD), TokenFill { color: Some({ let q: &dyn Query = g; planned_value!("a choice of Color outside its body", Color::White => q.state().chosen_color(q.linked(item)).expect("it chose one")) }), creature_type: Some({ let q: &dyn Query = g; planned_value!("a choice of CreatureType outside its body", Subtype::Plains => q.state().chosen_subtype(q.linked(item)).expect("it chose one")) }), power_toughness: Some(({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Charge))).sum::<i64>() }, { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Charge))).sum::<i64>() })), abilities_of: None }))], Arrival { tapped: false, counters: &[], states: &[] })?);
g.end_sentence();
hoisted_5.extend(created_3.clone());
}
g.end_sentence();
 Ok(()) }
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_0_enters(g, entering)?; ability_1_enters(g, entering)?;  Ok(()) }

pub static CARD: CardDef = CardDef { name: "Riptide Replicator", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_2, "{4}, {T}: Create an X/X creature token of the chosen color and type, where X is the number of charge counters on this artifact.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(entering_all), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_4::CARD];
