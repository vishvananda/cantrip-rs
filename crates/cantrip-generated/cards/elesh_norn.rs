// Elesh Norn // The Argent Etchings
// status: ready
// (As this Saga enters and after your draw step, add a lore counter.)
// I — Incubate 2 five times, then transform all Incubator tokens you control.
// II — Creatures you control get +1/+1 and gain double strike until end of turn.
// III — Destroy all other permanents except for artifacts, lands, and Phyrexians. Exile this Saga, then return it to the battlefield (front face up).
// Vigilance
// Whenever a source an opponent controls deals damage to you or a permanent you control, that source's controller loses 2 life unless they pay {1}.
// {2}{W}, Sacrifice three other creatures: Exile Elesh Norn, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if true && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(source)) && matches!(*target, Target::Player(p) if Some(asker.controller) == Some(p)) { return Some(Fired { subject, data }); } }
if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Object(o) => o.0, _ => u32::MAX }]; if true && (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&q.controller(source)) && matches!(*target, Target::Object(o) if Some(asker.controller) == Some(q.controller(o))) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let amount_1 = { let q: &dyn Query = g; (2i64).max(0) as i32 }; let players_1 = { let q: &dyn Query = g; item.subject.object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_1 { g.lose_life(player, amount_1); }
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
let objects_3 = ({ let q: &dyn Query = g; ((moved_2.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_3 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_3 { let controller = g.state().obj(o).owner; let entered = g.put_onto_battlefield_transformed(o, controller, false)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_4(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature)) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Elesh Norn", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Praetor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever a source an opponent controls deals damage to you or a permanent you control, that source's controller loses 2 life unless they pay {1}.").targets(&[]),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(3, ability_2_4)), ability_2, "{2}{W}, Sacrifice three other creatures: Exile Elesh Norn, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.").targets(&[]).sorcery_speed()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::tokens::INCUBATOR,&cantrip_mechanics::tokens::PHYREXIAN];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let runs_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; for _ in 0..runs_0 {
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = { let q: &dyn Query = g; (5i64).max(0) as i16 }; cantrip_mechanics::tokens::incubate(g, player, amount)?; }
}
g.end_sentence();
for bound_4294967293 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Incubator)) && q.state().obj(id).token && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() } {
if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967293).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
}
g.end_sentence();
 Ok(()) }
fn ability_0_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_4 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfTurn;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::Pump((1i64) as i16, (1i64) as i16) };
let kind_4_1_0 = { let q: &dyn Query = g; EffectKind::AddKeywords(Keywords::of(&[Keyword::DoubleStrike])) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4); g.add_effect(source, object, kind_4_1_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }
fn ability_0_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_5 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && !((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Land) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Phyrexian)))))).collect::<Vec<ObjectId>>() }; let destroyed_5 = g.destroy_all(&objects_5);
g.end_sentence();
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_6 = g.exile_all(&objects_6);
g.end_sentence();
let player_7 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_7 = ({ let q: &dyn Query = g; ((moved_6.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Exile).collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_7 = { let rows_7: Vec<(ObjectId, PlayerId, bool)> = objects_7.iter().map(|&o| (o, player_7, false)).collect(); let came_7 = g.put_all_onto_battlefield_as(&rows_7)?; came_7.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "The Argent Etchings", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Saga]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(1)), power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[chapter(1, ability_0_0, "I — Incubate 2 five times, then transform all Incubator tokens you control.").targets(&[]),chapter(2, ability_0_1, "II — Creatures you control get +1/+1 and gain double strike until end of turn.").targets(&[]),chapter(3, ability_0_2, "III — Destroy all other permanents except for artifacts, lands, and Phyrexians. Exile this Saga, then return it to the battlefield .").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
