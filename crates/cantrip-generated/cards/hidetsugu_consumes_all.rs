// Hidetsugu Consumes All // Vessel of the All-Consuming
// status: ready
// Trample
// Whenever this creature deals damage, put a +1/+1 counter on it.
// Whenever this creature deals damage to a player, if it has dealt 10 or more damage to that player this turn, they lose the game.
// (As this Saga enters and after your draw step, add a lore counter.)
// I — Destroy each nonland permanent with mana value 1 or less.
// II — Exile all graveyards.
// III — Exile this Saga, then return it to the battlefield transformed under your control.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Land)) && (i64::from(q.mana_value(id))) <= (1i64))).collect::<Vec<ObjectId>>() }; let destroyed_0 = g.destroy_all(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; ((q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let moved_1 = g.exile_all(&objects_1);
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let player_3 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_3 = ({ let q: &dyn Query = g; ((moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_3 { let controller = player_3; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Hidetsugu Consumes All", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Destroy each nonland permanent with mana value 1 or less.").targets(&[]),chapter(2, ability_0_1, "II — Exile all graveyards.").targets(&[]),chapter(3, ability_0_2, "III — Exile this Saga, then return it to the battlefield transformed under your control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), source.0]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut counters_0: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for t in { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(Target::Object).collect::<Vec<Target>>() } { if let Target::Object(object) = t { counters_0.push((object, CounterKind::PlusOne, amount)); } }
let had_0: Vec<_> = counters_0.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_0); let applied_0: Vec<ObjectId> = counters_0.iter().zip(had_0).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) && subject.object().is_some_and(|candidate: ObjectId| ({ let (outer_subject, outer_data) = (subject, data); q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Damage { source, target, amount, kind, controller, known, .. } = event { let (source, amount, combat, controller) = (*source, *amount, kind.is_combat(), *controller); let dealt = DamageDealt { source, target: *target, amount, combat, controller, known: *known }; let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if q.state().obj(source).zone.is(ZoneKind::Battlefield) && Some(candidate) == Some(source) && matches!(*target, Target::Player(p) if Some(PlayerId(outer_data[1] as u8)) == Some(p)) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>() }) >= (10i64)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller); if !{ let q: &dyn Query = g; item.subject.object().is_some_and(|candidate: ObjectId| (q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Damage { source, target, amount, kind, controller, known, .. } = event { let (source, amount, combat, controller) = (*source, *amount, kind.is_combat(), *controller); let dealt = DamageDealt { source, target: *target, amount, combat, controller, known: *known }; let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if q.state().obj(source).zone.is(ZoneKind::Battlefield) && Some(candidate) == Some(source) && matches!(*target, Target::Player(p) if Some(PlayerId(item.data[1] as u8)) == Some(p)) { return Some(Fired { subject, data }); } }  None }).map(|f: Fired| i64::from(f.data[0])).sum::<i64>()) >= (10i64)) } { return Ok(()); } let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Vessel of the All-Consuming", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Ogre,Subtype::Shaman]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(12)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever this creature deals damage, put a +1/+1 counter on it.").targets(&[]).sums_per_subject(),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever this creature deals damage to a player, if it has dealt 10 or more damage to that player this turn, they lose the game.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
