// Agrus Kos, Eternal Soldier
// status: planned
// requires: a copy of a spell for each object it could target
// Vigilance
// Whenever Agrus Kos becomes the target of an ability that targets only it, you may pay {1}{R/W}. If you do, copy that ability for each other creature you control that ability could target. Each copy targets a different one of those creatures. ({R/W} can be paid with either {R} or {W}.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::BecameTarget { target: Target::Object(target), by, controller, .. } = event { let id = *target; let by = *by; let controller = *controller; let subject = Target::Object(id); let data = [by.0, u32::from(controller.0) | (q.state().stack.len().saturating_sub(1) as u32) << 8]; if Some(asker.source) == Some(id) && !q.state().stack.iter().any(|i| i.kind == StackKind::Spell && i.object == by) && Some(asker.source).is_some_and(|it: ObjectId| q.state().stack.last().is_some_and(|i| { let n = q.state().targets(i).len(); n > 0 && (0..n).all(|k| q.state().target(i, k) == Target::Object(it)) })) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [Some((Mana::R, Mana::W)),None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
planned!("a-copy-of-a-spell-for-each-object-it-could-target" => cantrip_mechanics::copy::ability_on_stack(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, { let q: &dyn Query = g; Target::Ability(q.state().stack.get((item.data[1] >> 8) as usize).map_or(u32::MAX, |e| e.serial)) }, { let q: &dyn Query = g; (1i64).max(0) as u32 }, NewTargets::Each({ let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))))).collect::<Vec<ObjectId>>() }))?;);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Agrus Kos, Eternal Soldier", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Soldier]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::BECAME_TARGET.0), ability_1_event), ability_1, "Whenever Agrus Kos becomes the target of an ability that targets only it, you may pay {1}{R/W}. If you do, copy that ability for each other creature you control that ability could target. Each copy targets a different one of those creatures.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
