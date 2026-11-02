// Rottenmouth Viper
// status: planned
// requires: object reference Bound
// requires: ways to pay chosen for every time first, then all paid at once
// As an additional cost to cast this spell, you may sacrifice any number of nonland permanents. This spell costs {1} less to cast for each permanent sacrificed this way.
// Whenever this creature enters or attacks, put a blight counter on it. Then for each blight counter on it, each opponent loses 4 life unless that player sacrifices a nonland permanent of their choice or discards a card.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(q.chars(id).is(CardType::Land)) && (q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_1: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_1.push((object, CounterKind::Blight, amount)); } }
let had_1: Vec<_> = counters_1.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_1); let applied_1: Vec<ObjectId> = counters_1.iter().zip(had_1).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
let mut runs_2: u32 = { let q: &dyn Query = g; ((item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| i64::from(q.state().obj(o).counters(CounterKind::Blight))).sum::<i64>()) as u32 }; while runs_2 > 0 { runs_2 -= 1; let mut again_2 = false;
let mut players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_3.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut guard_3_0 = vec![false; players_3.len()];
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { planned!("ways-to-pay-chosen-for-every-time-first-then-all-paid-at-once", PayCost, "cost" = "{\"parts\":[{\"Choice\":[{\"parts\":[{\"Sacrifice\":{\"choose\":{\"Exactly\":{\"args\":1,\"kind\":\"Constant\"}},\"objects\":{\"args\":{\"args\":{\"args\":\"Land\",\"kind\":\"CardType\"},\"kind\":\"Not\"},\"kind\":\"Battlefield\"}}}]},{\"parts\":[{\"Discard\":{\"count\":{\"args\":1,\"kind\":\"Constant\"},\"filter\":{\"kind\":\"Any\"},\"random\":false}}]}]}]}", "optional" = "true", "player" = { let q: &dyn Query = g; planned_value!("object reference Bound", None::<ObjectId> => bound_0) });
    } 
g.end_sentence();
 for (index_3, bound_0) in players_3.iter().copied().enumerate() { if { let q: &dyn Query = g; !(planned_value!("depends on an instruction with a gap", false => result_1)) } { guard_3_0[index_3] = true;
let amount_5 = { let q: &dyn Query = g; (4i64).max(0) as i32 }; let players_5 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() };
for player in players_5 { g.lose_life(player, amount_5); }
   }
 } 
g.end_sentence();
if !again_2 { break; } }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Rottenmouth Viper", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Snake]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0 | EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever this creature enters or attacks, put a blight counter on it. Then for each blight counter on it, each opponent loses 4 life unless that player sacrifices a nonland permanent of their choice or discards a card.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, additional_cost: Some(Cost { variable: Some(VariableCost::SacrificeAnyNumber { filter: ability_0_0, less_each: 1 }), ..Cost::NONE }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
