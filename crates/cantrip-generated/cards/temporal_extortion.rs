// Temporal Extortion
// status: planned
// requires: player reference FirstWilling
// When you cast this spell, any player may pay half their life, rounded up. If a player does, counter Temporal Extortion.
// Take an extra turn after this one.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let willing_0: Option<PlayerId> = planned_value!("player reference FirstWilling", None::<PlayerId> => cantrip_mechanics::choose::first_willing(g, { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }, asker.source)?);
let mut paid_3 = false; for bound_1000004 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_5 = { let q: &dyn Query = g; Some(bound_1000004).expect("a reference that always names a player") }; let paid_5 = g.may_pay(payer_5, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { life: ({ let q: &dyn Query = g; (willing_0.map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))).saturating_add(1).div_euclid(2) }).clamp(0, i64::from(u32::MAX)) as u32, ..Cost::NONE }), &PAY_QUESTION)?;
if paid_5 { paid_3 = true; break; } }

g.end_sentence();
if { let q: &dyn Query = g; ((paid_3 as i64) != 0) } {
let objects_6 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; cantrip_api::instruction(g, |g| { for spell in objects_6 { g.counter(spell); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let times = 1u32; g.add_extra_turns(player, times); }
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Temporal Extortion", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 0, 4, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_1), abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "When you cast this spell, any player may pay half their life, rounded up. If a player does, counter Temporal Extortion.").targets(&[]).functions_in(ZoneKinds::of(&[ZoneKind::Stack]))], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
