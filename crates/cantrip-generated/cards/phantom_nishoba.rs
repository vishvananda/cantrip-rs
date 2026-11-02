// Phantom Nishoba
// status: planned
// requires: engine: no static prevention with an effect besides
// Trample
// This creature enters with seven +1/+1 counters on it.
// Whenever this creature deals damage, you gain that much life.
// If damage would be dealt to this creature, prevent that damage. Remove a +1/+1 counter from this creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (7i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_1 = { let q: &dyn Query = g; (i64::from(item.data[0])).min(i32::MAX.into()) as i32 }; let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.gain_life(player, amount_1); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_3_then_steps(g: &mut dyn Game, asker: &Asker, subject: Target, data: [u32; 2], done_prevented: u16) -> Result<(), Suspend> { let mut removed_2: i64 = 0;
let objects_2_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() };
let amount_2_0 = { let q: &dyn Query = g; (1i64).max(0) as i16 }; cantrip_api::instruction(g, |g| { for object in objects_2_0 { removed_2 += i64::from(g.remove_counters(object, CounterKind::PlusOne, amount_2_0)); } });
 Ok(()) }
#[allow(unused_variables, dead_code)] fn ability_3_then(g: &mut dyn Game, asker: &Asker, parts: &[Prevented]) { let Some(first) = parts.first() else { return }; let prevented = Prevented::total(parts).min(u16::MAX.into()) as u16; let e = &Prevented { amount: prevented, ..*first }; let q: &dyn Query = g; let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; let _ = ability_3_then_steps(g, asker, subject, data, prevented); }
#[allow(unused_variables)] fn ability_3_applies(q: &dyn Query, asker: &Asker, e: &cantrip_api::DamageEvent) -> bool { { let (source, target, amount, combat) = (e.source, &e.target, e.amount, e.combat); let subject = *target; let data = [u32::from(amount), source.0]; if true && true && matches!(*target, Target::Object(o) if Some(asker.source) == Some(o)) { return true; } }
 false }
planned_item!("engine-no-static-prevention-with-an-effect-besides" => preventions: PreventionDef { hint: "If damage would be dealt to this creature, prevent that damage. Remove a +1/+1 counter from this creature.", applies: ability_3_applies, amount: None, then: Some(ability_3_then) });

pub static CARD: CardDef = CardDef { name: "Phantom Nishoba", token_words: &[], oracle: true, cost: ManaCost { generic: 5, x: 0, symbols: [1, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Cat,Subtype::Beast,Subtype::Spirit]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(0), toughness: Some(0), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever this creature deals damage, you gain that much life.").targets(&[]).sums_per_subject()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
