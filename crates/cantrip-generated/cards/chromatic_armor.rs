// Chromatic Armor
// status: planned
// requires: a choice of Color made again
// requires: engine: no rule on an ability's X
// Enchant creature
// As this Aura enters, choose a color.
// This Aura enters with a sleight counter on it.
// Prevent all damage that would be dealt to enchanted creature by sources of the last chosen color.
// {X}: Put a sleight counter on this Aura and choose a color. X is the number of sleight counters on this Aura.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && (q.chars(id).is(CardType::Creature))) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let player_0 = entering.controller; let Answer::Colors(colors_0) = g.ask(player_0, RequestKind::ChooseColors { source: entering.object, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_0 = colors_0[0];
entering.chosen_color = Some(chosen_0);
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::Sleight as usize] = entering.counters[CounterKind::Sleight as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && q.state().chosen_color(q.linked_to(asker)).is_some_and(|chosen| q.chars(source).colors.has(chosen)) && matches!(*target, Target::Object(o) if Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_to) == Some(o)) { return true; } }
 false }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_2: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_2.push((object, CounterKind::Sleight, amount)); } }
let had_2: Vec<_> = counters_2.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_2); let applied_2: Vec<ObjectId> = counters_2.iter().zip(had_2).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let Answer::Colors(colors_3) = g.ask(player_3, RequestKind::ChooseColors { source: asker.source, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_3 = colors_3[0];
planned!("a-choice-of-color-made-again" => let by = { let q: &dyn Query = g; q.linked(item) }; g.remember_chosen(by, ChosenValue::Color(chosen_3)););
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-rule-on-an-ability-s-x" => #[allow(unused_variables)] fn ability_4_x_is(q: &dyn Query, asker: &Asker) -> i64 { (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Sleight))).sum::<i64>() });
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_1_enters(g, entering)?; ability_2_enters(g, entering)?;  Ok(()) }

pub static CARD: CardDef = CardDef { name: "Chromatic Armor", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[planned_value!("engine: no rule on an ability's X", ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_4, "{X}: Put a sleight counter on this Aura and choose a color. X is the number of sleight counters on this Aura.").targets(&[]) => ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_4, "{X}: Put a sleight counter on this Aura and choose a color. X is the number of sleight counters on this Aura.").targets(&[]).x_is(ability_4_x_is))], statics: &[], replacements: &[], preventions: &[PreventionDef { hint: "prevents all damage that would be dealt to enchanted creature by sources of the last chosen color", applies: ability_3_applies, amount: None }], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(entering_all), modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
