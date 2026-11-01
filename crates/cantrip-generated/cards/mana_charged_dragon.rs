// Mana-Charged Dragon
// status: ready
// Flying, trample
// Join forces — Whenever this creature attacks or blocks, each player starting with you may pay any amount of mana. This creature gets +X/+0 until end of turn, where X is the total amount of mana paid this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_0 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_0.iter().position(|&p| Some(p) == first_0) { players_0.rotate_left(at); }
let mut each_2 = vec![Default::default(); players_0.len()];
let mut done_total_2: i64 = 0;
let mut each_3 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { done_1 += { i64::from(g.may_pay_x(player, asker.source, asker.source, cost("{X}"), &PAY_QUESTION)?.unwrap_or(0)) }; }
 each_2[index_0] = done_1;
done_total_2 += done_1;
each_3[index_0] = done_1;
   } 
g.end_sentence();
let objects_4 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_4 = { let q: &dyn Query = g; Some(asker.source) }; let duration_4 = Duration::EndOfTurn;
let kind_4_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((done_total_2 as i64)).clamp(-32768, 32767) as i16, (0i64) as i16) };
if let Some(source) = source_4 { cantrip_api::instruction(g, |g| { for &object in &objects_4 { g.add_effect(source, object, kind_4_0_0, duration_4);  } }); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { blocker, attacker, declared: true } = event { let id = *blocker; let subject = Target::Object(id); let data = [attacker.0, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_5: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let first_5 = { let q: &dyn Query = g; Some(asker.controller) }; if let Some(at) = players_5.iter().position(|&p| Some(p) == first_5) { players_5.rotate_left(at); }
let mut each_7 = vec![Default::default(); players_5.len()];
let mut done_total_7: i64 = 0;
let mut each_8 = vec![Default::default(); players_5.len()];
 for (index_5, bound_0) in players_5.iter().copied().enumerate() { let mut done_6: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { done_6 += { i64::from(g.may_pay_x(player, asker.source, asker.source, cost("{X}"), &PAY_QUESTION)?.unwrap_or(0)) }; }
 each_7[index_5] = done_6;
done_total_7 += done_6;
each_8[index_5] = done_6;
   } 
g.end_sentence();
let objects_9 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_9 = { let q: &dyn Query = g; Some(asker.source) }; let duration_9 = Duration::EndOfTurn;
let kind_9_0_0 = { let q: &dyn Query = g; EffectKind::Pump(((done_total_7 as i64)).clamp(-32768, 32767) as i16, (0i64) as i16) };
if let Some(source) = source_9 { cantrip_api::instruction(g, |g| { for &object in &objects_9 { g.add_effect(source, object, kind_9_0_0, duration_9);  } }); }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Mana-Charged Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Join forces — Whenever this creature attacks or blocks, each player starting with you may pay any amount of mana. This creature gets +X/+0 until end of turn, where X is the total amount of mana paid this way.").targets(&[]),triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_3_event), ability_3, "Join forces — Whenever this creature attacks or blocks, each player starting with you may pay any amount of mana. This creature gets +X/+0 until end of turn, where X is the total amount of mana paid this way.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
