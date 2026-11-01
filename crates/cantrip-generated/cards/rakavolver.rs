// Rakavolver
// status: planned
// requires: object filter KickedWith
// requires: two costs a spell kicks with
// Kicker {1}{W} and/or {U} (You may pay an additional {1}{W} and/or {U} as you cast this spell.)
// If this creature was kicked with its {1}{W} kicker, it enters with two +1/+1 counters on it and with "Whenever this creature deals damage, you gain that much life."
// If this creature was kicked with its {U} kicker, it enters with a +1/+1 counter on it and with flying.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn granted_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && true { return Some(Fired { subject, data }); } }
 None }
fn granted_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (i64::from(item.data[0])).clamp(0, 2147483647) as i32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.gain_life(player, amount_3); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| planned_value!("object filter KickedWith", false => q.state().obj(candidate).kicked_with(kicker))) } {
let more = { let q: &dyn Query = g; (2i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
static WHAT_4: EntersAs = EntersAs { abilities: &[0], keywords: Keywords::of(&[]), ..EntersAs::NONE }; entering.becomes = Some(Becomes { what: &WHAT_4, card: { let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, words: g.words(entering.object) });
}
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_3_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().is_some_and(|candidate: ObjectId| planned_value!("object filter KickedWith", false => q.state().obj(candidate).kicked_with(kicker))) } {
let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
static WHAT_7: EntersAs = EntersAs { abilities: &[], keywords: Keywords::of(&[Keyword::Flying]), ..EntersAs::NONE }; entering.becomes = Some(Becomes { what: &WHAT_7, card: { let q: &dyn Query = g; { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) } }, words: g.words(entering.object) });
}
 Ok(()) }
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_2_enters(g, entering)?; ability_3_enters(g, entering)?;  Ok(()) }
planned_item!("two-costs-a-spell-kicks-with" => second_kicker: kicker(ManaCost { generic: 0, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }));

pub static CARD: CardDef = CardDef { name: "Rakavolver", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Volver]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), granted_2_event), granted_2, "Whenever this creature deals damage, you gain that much life.").targets(&[]).sums_per_subject()], enters: Some(entering_all), modal: None, enchant: None, kicker: kicker(ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
