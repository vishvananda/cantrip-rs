// Strict Proctor
// status: planned
// requires: trigger event CausesTrigger
// Flying
// Whenever a permanent entering causes a triggered ability to trigger, counter that ability unless its controller pays {2}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }; let paid_0 = g.may_pay(payer_0, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?;
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let items_1: Vec<(ObjectId, usize)> = { let t = Target::Ability(item.data[0]); let stack = &g.state().stack; stack.iter().position(|e| Target::Ability(e.serial) == t && e.kind != StackKind::Spell).map(|at| (stack[at].object, at)).into_iter().collect() }; g.counter_items(&items_1);
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");
planned_item!("trigger-event-causes-trigger" => matcher: #[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Triggered { source, ability, cause } = event { let id = *source; let subject = Target::Object(id); let on_stack = cantrip_mechanics::triggers::triggered_ability_on_stack(q, event); let data = [on_stack.map_or(0, |i: StackItem| i.serial), on_stack.map_or(0, |i: StackItem| u32::from(i.controller.0))]; let cause = &*cause; if (if let Event::ZoneChange { old, new, from, to, .. } = cause { let id = *new; let subject = Target::Object(id); let data = [new.0, u32::MAX]; to.is(ZoneKind::Battlefield) && true } else { false }) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Strict Proctor", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Spirit,Subtype::Cleric]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event CausesTrigger", EventMask(0) => EventMask(EventMask::TRIGGERED.0)), ability_1_event), ability_1, "Whenever a permanent entering causes a triggered ability to trigger, counter that ability unless its controller pays {2}.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
